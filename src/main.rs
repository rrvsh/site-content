use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
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
    let mut args = env::args().skip(1).collect::<Vec<String>>();
    let allow_push = args.iter().any(|arg| arg == "--push");
    args.retain(|arg| arg != "--push");

    if args.first().map(|arg| arg.as_str()) == Some("--finalize") {
        args.remove(0);
        let path = args.get(0).ok_or("missing path")?;
        let date = args.get(1).ok_or("missing date")?;
        let commit_stamp = args.get(2).ok_or("missing commit stamp")?;
        let path = Path::new(path);
        println!("Generating frontmatter (if needed) and preparing git commit...");
        ensure_frontmatter(path, date)?;
        commit_and_push(path, commit_stamp, allow_push)?;
        return Ok(());
    }

    let now = Local::now();
    let today = now.date_naive();
    let date = today.format("%Y-%m-%d").to_string();
    let commit_stamp = now.format("%Y-%m-%d %H:%M").to_string();
    let year = today.format("%Y").to_string();
    let month = today.format("%B").to_string().to_lowercase();

    let relative_path = PathBuf::from(year).join(month);
    let path = next_entry_path(&relative_path, &date)?;

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
        .arg("+startinsert")
        .arg(format!("+{WRITING_LINE}"))
        .arg(path)
        .status()?;

    if !status.success() {
        return Err("nvim exited with a non-zero status".into());
    }

    println!("Editor closed. Backgrounding frontmatter generation and git commit...");
    let exe = env::current_exe()?;
    let mut command = Command::new(exe);
    command
        .arg("--finalize")
        .arg(path)
        .arg(&date)
        .arg(&commit_stamp);
    if allow_push {
        command.arg("--push");
    }
    command.spawn()?;

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

fn commit_and_push(
    path: &Path,
    commit_stamp: &str,
    allow_push: bool,
) -> Result<(), Box<dyn Error>> {
    let status = Command::new("git").arg("add").arg(path).status()?;
    if !status.success() {
        return Err("git add failed".into());
    }

    let status = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(commit_stamp)
        .status()?;
    if !status.success() {
        return Err("git commit failed".into());
    }

    if allow_push {
        let status = Command::new("git").arg("push").status()?;
        if !status.success() {
            return Err("git push failed".into());
        }
    }

    Ok(())
}

fn next_entry_path(base_dir: &Path, date: &str) -> Result<PathBuf, Box<dyn Error>> {
    fs::create_dir_all(base_dir)?;

    let mut max_index: Option<u32> = None;
    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }
        let filename = match path.file_stem().and_then(|name| name.to_str()) {
            Some(name) => name,
            None => continue,
        };
        let Some(rest) = filename.strip_prefix(&format!("{date}-")) else {
            continue;
        };
        if rest.len() != 2 || !rest.chars().all(|ch| ch.is_ascii_digit()) {
            continue;
        }
        let index: u32 = rest.parse()?;
        max_index = Some(max_index.map_or(index, |current| current.max(index)));
    }

    let next_index = max_index.map_or(0, |index| index + 1);
    if next_index > 99 {
        return Err("daily entry index exceeded 99".into());
    }
    let filename = format!("{date}-{next_index:02}.md");
    Ok(base_dir.join(filename))
}
