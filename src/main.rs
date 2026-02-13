use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const WRITING_LINE: usize = 8;
const ALWAYS_TAG: &str = "daily-blog";

#[derive(Debug, Default, Serialize, Deserialize)]
struct Frontmatter {
    title: Option<String>,
    slug: Option<String>,
    date: Option<String>,
    tags: Option<Vec<String>>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let today = Local::now().date_naive();
    let date = today.format("%Y-%m-%d").to_string();
    let year = today.format("%Y").to_string();
    let month = today.format("%B").to_string().to_lowercase();

    let relative_path = PathBuf::from(year)
        .join(month)
        .join(format!("{date}.md"));
    let path = Path::new(&relative_path);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    if !path.exists() {
        let content = format!(
            "---\ntitle:\nslug:\ndate: {date}\ntags: []\n---\n\n"
        );
        fs::write(path, content)?;
    }

    let status = Command::new("nvim")
        .arg(format!("+{WRITING_LINE}"))
        .arg(path)
        .status()?;

    if !status.success() {
        return Err("nvim exited with a non-zero status".into());
    }

    ensure_frontmatter(path, &date)?;
    commit_and_push(path, &date)?;

    Ok(())
}

fn ensure_frontmatter(path: &Path, date: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(path)?;
    let (raw_frontmatter, body) = split_frontmatter(&contents);

    let mut frontmatter: Frontmatter = if raw_frontmatter.trim().is_empty() {
        Frontmatter {
            date: Some(date.to_string()),
            ..Frontmatter::default()
        }
    } else {
        serde_yaml::from_str(&raw_frontmatter)?
    };

    if frontmatter.date.as_deref().unwrap_or("").trim().is_empty() {
        frontmatter.date = Some(date.to_string());
    }

    let needs_title = is_blank(&frontmatter.title);
    let needs_slug = is_blank(&frontmatter.slug);
    let needs_tags = frontmatter
        .tags
        .as_ref()
        .map(|tags| tags.is_empty())
        .unwrap_or(true);

    let trimmed_body = body.trim();
    if needs_title || needs_slug || needs_tags {
        let generated = generate_with_opencode(trimmed_body)?;
        if needs_title {
            frontmatter.title = Some(generated.title);
        }
        if needs_slug {
            frontmatter.slug = Some(generated.slug);
        }
        if needs_tags {
            frontmatter.tags = Some(generated.tags);
        }
    }

    let output = format_frontmatter(&frontmatter, &body);
    fs::write(path, output)?;
    Ok(())
}

fn split_frontmatter(contents: &str) -> (String, String) {
    let mut lines = contents.lines();
    if lines.next() != Some("---") {
        return (String::new(), contents.to_string());
    }

    let mut front_lines = Vec::new();
    let mut body_lines = Vec::new();
    let mut in_front = true;

    for line in lines {
        if in_front && line == "---" {
            in_front = false;
            continue;
        }
        if in_front {
            front_lines.push(line);
        } else {
            body_lines.push(line);
        }
    }

    (front_lines.join("\n"), body_lines.join("\n"))
}

fn format_frontmatter(frontmatter: &Frontmatter, body: &str) -> String {
    let title = frontmatter
        .title
        .as_deref()
        .unwrap_or("")
        .trim();
    let slug = frontmatter
        .slug
        .as_deref()
        .unwrap_or("")
        .trim();
    let date = frontmatter
        .date
        .as_deref()
        .unwrap_or("")
        .trim();
    let tags = frontmatter.tags.clone().unwrap_or_default();
    let tag_string = if tags.is_empty() {
        "".to_string()
    } else {
        tags.join(", ")
    };
    let body = body.trim_end();

    if tag_string.is_empty() {
        format!(
            "---\ntitle: {title}\nslug: {slug}\ndate: {date}\ntags: []\n---\n\n{body}\n",
        )
    } else {
        format!(
            "---\ntitle: {title}\nslug: {slug}\ndate: {date}\ntags: [{tag_string}]\n---\n\n{body}\n",
        )
    }
}

fn is_blank(value: &Option<String>) -> bool {
    value.as_deref().map(|text| text.trim().is_empty()).unwrap_or(true)
}

#[derive(Debug, Deserialize)]
struct GeneratedMetadata {
    title: String,
    slug: String,
    tags: Vec<String>,
}

fn generate_with_opencode(body: &str) -> Result<GeneratedMetadata, Box<dyn Error>> {
    let prompt_body: String = body.chars().take(2000).collect();
    let prompt = format!(
        "Return a JSON object only (no markdown) with keys title, slug, tags. \
Title should be short and descriptive. Slug should be lowercase hyphenated. \
Tags should be an array of lowercase strings and must include \"{ALWAYS_TAG}\".\n\n\
Content:\n{prompt_body}\n"
    );

    let output = Command::new("opencode")
        .arg("run")
        .arg("--format")
        .arg("json")
        .arg(prompt)
        .output()?;

    if !output.status.success() {
        return Err("opencode run failed".into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    let response = extract_text_response(&stdout)?;
    let mut generated: GeneratedMetadata = serde_json::from_str(response.trim())?;
    if !generated.tags.iter().any(|tag| tag == ALWAYS_TAG) {
        generated.tags.push(ALWAYS_TAG.to_string());
    }

    Ok(generated)
}

fn extract_text_response(stdout: &str) -> Result<String, Box<dyn Error>> {
    let mut combined = String::new();
    for line in stdout.lines() {
        let value: Value = match serde_json::from_str(line) {
            Ok(value) => value,
            Err(_) => continue,
        };
        if value.get("type").and_then(Value::as_str) != Some("text") {
            continue;
        }
        if let Some(text) = value
            .get("part")
            .and_then(|part| part.get("text"))
            .and_then(Value::as_str)
        {
            combined.push_str(text);
        }
    }

    if combined.trim().is_empty() {
        return Err("opencode returned no text response".into());
    }

    Ok(combined)
}

fn commit_and_push(path: &Path, date: &str) -> Result<(), Box<dyn Error>> {
    let status = Command::new("git").arg("add").arg(path).status()?;
    if !status.success() {
        return Err("git add failed".into());
    }

    let status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(date)
        .status()?;
    if !status.success() {
        return Err("git commit failed".into());
    }

    let status = Command::new("git").arg("push").status()?;
    if !status.success() {
        return Err("git push failed".into());
    }

    Ok(())
}
