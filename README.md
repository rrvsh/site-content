# <https://rrv.sh> content repo

Files with a parseable date, title, content, and slug will be published live on the site. Files without will be ignored. Organisation is mostly for my comfort, and currently is inconsistent, but in general I try and put articles written around the same time next to each other.

## Daily template CLI

This repo ships a Rust CLI that scaffolds a daily post, opens it in Neovim, then fills in any missing frontmatter after you exit the editor. It also commits and pushes the new entry automatically.

### What it does

- Creates a file at `YYYY/month/YYYY-MM-DD.md` (month is lowercase).
- Inserts the frontmatter template from `template.md` with today’s date.
- Opens Neovim on the writing line in insert mode.
- After you exit Neovim, it fills missing `title`, `slug`, or `tags` using `opencode`.
- Always includes the `daily-blog` tag if tags are auto-generated.
- Runs `git add <file>`, `git commit -m "YYYY-MM-DD"`, and `git push`.

### Requirements

- Nix (recommended)
- `opencode` in PATH (installed via Nix dev shell below)
- `git` configured with an authenticated remote
- Neovim

### Run with Nix

```bash
nix develop
cargo run
```

The dev shell provides Rust, Neovim, and `opencode`.

### Run without Nix

You can run it locally as long as you have Rust, Neovim, and `opencode` installed.

```bash
cargo build
cargo run
```

### Frontmatter rules

- If `title` is blank, the CLI asks `opencode` to generate a short descriptive title.
- If `slug` is blank, the CLI asks `opencode` to generate a lowercase hyphenated slug.
- If `tags` is empty or missing, the CLI asks `opencode` to generate tags and ensures `daily-blog` is included.
- If any of these fields are already present, the CLI leaves them unchanged.

### Notes

- The post-processing runs in a background child process after you exit Neovim.
- If `opencode` fails or returns an invalid response, the CLI exits with an error and does not attempt to commit.
