---
title: Live Syncing to a Git Repository with a VS Code Extension
slug: git-livesync
date: 2025-02-10
tags: [git, vscode]
---

> Psst! The source code talked about below is up at <https://github.com/bwfiq/git-livesync> :)

## Foreword

As mentioned in the last post, I keep my notes in git repositories. I originally used [Obsidian](https://obsidian.md/) for years as my note-taking application of choice after migrating away from Google Keep, using the vast library of community plugins (namely [obsidian-livesync](https://github.com/vrtmrz/obsidian-livesync) and [obsidian-git](https://github.com/Vinzent03/obsidian-git)) to back up and sync my notes on an interval to my 3 remotes; GitHub, my private [Gitea](https://github.com/go-gitea/gitea) instance for my private "second brain" type notes, and my [Otterwiki](https://github.com/redimp/otterwiki/) instance (a wiki that runs on a git server of markdown files).

A recent issue I've run into is that since I started working my first big boy job, I've been unable to download or install any software. I already foresaw this, though, as the main reason I use Obsidian (other than how great it is as a note-taking app) is that all the notes are stored in a very transparent directory structure as markdown files. I simply spun up a [code-server](https://github.com/coder/code-server) instance, cloned my notes repository, and was off to the races.

The actual issue was me missing my automatic sync. Thankfully, I almost immediately found [GitDoc](https://github.com/lostintangent/gitdoc), which I mentioned in the last post as something I was looking into to replicate the live sync functionality I was used to.

I used it quite happily for a few hours, until I realised it stopped committing my changes randomly and would sometimes not even work even on startup. I dug through the VS Code logs, but none of my extensions were logging any warnings or errors.

Of course, I did the sane thing at this point, and manually committed my changes in a normal, human way.

...

Just kidding. Here's how I made my own extension to watch for file changes in VS Code and automatically syncs them to a configured remote.

_By the way, when writing this post, I found a [Github Issue](https://github.com/lostintangent/gitdoc/issues/88) that gave me more information about why GitDoc wasn't working for me; its latest version is for some reason incompatible with [Foam](https://github.com/foambubble/foam), which I use to get Obsidian-like functionality in VS Code. Unfortunately, it seems like there is no resolution yet._

## Typescript

So, some background. I grew up on C and Python, and mostly used C in my education, with the occasional module teaching me JS or Java. Even my first dev job in 2022 was as a Unity developer, so I continued in C#. I also recently picked up Rust for a side project.

If you have ever worked with VS Code or its extensions, you know where this is going. As soon as I cloned the vs code tutorial repository, I figured out I would have to use Typescript. As someone who really disliked using Javascript, I dreaded this (slightly).

Luckily though this project taught me that the general positive opinion on Typescript by its users is not unfounded. I lowkey loved using it and am excited to try and use it in an actual web dev project.

Anyway, on to the actual project.

## Writing git-livesync

Since I didn't have any experience with Typescript or the [VS Code API](https://code.visualstudio.com/api/references/vscode-api), I started with a goal of a prototype that could live sync any changed files to a git repository, regardless of the implementation.

### The MVP

I quickly followed the VS Code extension tutorial, which basically taught me jack shit except the file structure of VS Code extensions. (No hate to the authors, though. The people at VS Code write damn good tutorials; I learned a lot from their Django one)

Regardless, I started the project by using the [yeoman generator for vs code extensions](https://www.npmjs.com/package/generator-code) with:

```bash
yo code .
```

_Take note you need to have [NVM and NPM](https://nodejs.org/en/download) set up on your local machine._

This set up my local development environment with the proper project structure. I'm lazy and didn't bother with writing unit tests for this project, so the only files we are concerned with are `package.json` and `src/extension.ts`.

`package.json` is a node thing that defines a lot of information about the project, and VS Code extends this by including extension metadata for the marketplace and the extension view, as well as using it to add [Activation Events](https://code.visualstudio.com/api/references/activation-events) and [configuration settings](https://code.visualstudio.com/api/references/contribution-points#contributes.configuration).

`src/` is gonna contain all our typescript files that define the extension's logic, and `extension.ts` is compiled to `out/extension.js`, which is the entrypoint defined in our `package.json` for the extension.

_As an aside, I have recently adopted the [Conventional Commit](https://www.conventionalcommits.org/) syntax for my commits, hoping that this will improve both readability of my repos and also force me to think more carefully about what I commit. This first task of this project however, was totally not in sync with this philosophy; I just wrote the entire intended logic of the extension, tested it, and pushed it. Later commits are better separated._

We use the VS Code API to watch the filesystem for changes, and this does not change throughout the project.

```typescript
// Creates a file system watcher
const watcher = vscode.workspace.createFileSystemWatcher("**/*");

// Run our logic on events sent from the watcher
watcher.onDidChange((e) => {
  handleFileEvent(e, "changed");
});

watcher.onDidCreate((e) => {
  handleFileEvent(e, "created");
});

watcher.onDidDelete((e) => {
  handleFileEvent(e, "deleted");
});
context.subscriptions.push(watcher);
```

For now, I ran a simple terminal window that runs my git commands, though I will be changing this later to use [simple-git](https://www.npmjs.com/package/simple-git).

```typescript
const handleFileEvent = (event: vscode.Uri, action: string) => {
  // We'll first get the path of the file that was modified.
  const file_path = event.fsPath;
  const relative_file_path = path.relative(
    vscode.workspace.workspaceFolders[0].uri.fsPath,
    file_path,
  );
  // This handles the .gitignore exclusions; it will be explained in the next section.
  if (!ig.ignores(relative_file_path)) {
    const date = new Date();
    const commit_message = `${action} ${relative_file_path} at ${date.toISOString()}`;
    // We create a terminal and send the commands.
    const terminal = vscode.window.createTerminal("Git LiveSync");
    terminal.sendText(`cd ${vscode.workspace.workspaceFolders[0].uri.fsPath}`);
    terminal.sendText(
      `git pull && git add . && git commit -m "${commit_message}" && git push`,
    );
  }
};
```

Finally, this MVP includes one of the key features of respecting excluded files and folders from the .gitignore. This ensures we only initiate a commit-and-sync when only files we want to commit are modified. We'll use the ignore module for this.

```typescript
import ignore from "ignore";
// We first retrieve the .gitignore...
const gitignorePath = path.join(
  vscode.workspace.workspaceFolders[0].uri.fsPath,
  ".gitignore",
);
// then extract the exclusion patterns from it.
let git_ignore_patterns: string[] = [];
if (fs.existsSync(gitignorePath)) {
  const gitignoreContent = fs.readFileSync(gitignorePath, "utf8");
  git_ignore_patterns = gitignoreContent
    .split("\n")
    .filter((line) => line && !line.startsWith("#"));
}
git_ignore_patterns.push(".git"); // Include the .git/ directory as well.
// We then add these patterns to our ignorer to use.
const ig = ignore().add(git_ignore_patterns);
```

_You can see the extension.ts at [this point here](https://github.com/bwfiq/git-livesync/blob/9965b58500c0cb786521090609ce2d7babb318bd/src/extension.ts)._

### Refactoring

With this simple implementation of the extension's goals, we now have a working MVP. Whenever a file is changed, created, or deleted, the extension commits-and syncs (aka pulling changes from the remote before committing and pushing).

Here's a non-exhaustive list of the problems right now before we can use this extension, though:

- There is no cooldown on the file watching or terminal functionality; it will spam commits as soon as any file is changed for every single change detected.
- The current implementation of the git command runner spawns new VS Code terminals that are visible in the terminal view in VS Code. This is technically not an issue, but it's ugly.
- This functionality does not respect what workspace you are in, instead running the commands on any change in any workspace, irrespective of if you want it to or if the workspace even has a git repository.

I wanna fix these issues, but first, I decided to refactor the current logic into a collection of source files. We will handle git operations using a GitHandler class in `src/gitHandler.ts`, exclusion patterns using a IgnoreHandler class in `src/ignoreHandler.ts`, and file system watching using a Watcher class in `src/watcher.ts`.

_For the sake of brevity, I won't include the refactor here, but you can see [the commit here](https://github.com/bwfiq/git-livesync/commit/55dcb8abfc2a75403e37522589f37eadf8a37a03)._

I was tempted to improve the code during the refactoring process as I am wont to do, but in keeping spirit with Conventional Commit syntax, I refrained from doing so. However, we are now in a great position to begin working on the issues that we have identified.

### Improving

My next step was to tackle the issues of workspace-specific functionality and the cooldown between running git commands, which can both be solved with VS Code configuration settings. I want the extension to be disabled by default, and have the user enable it in the workspaces they want to have live synced (exactly like GitDoc!).

Doing this for a VS Code extension necessitates first defining the config settings in the `package.json`:

```json
    "configuration": [
        {
            // We can have our settings under different headings, but we just want them to be under the main heading so the title will be the name of the extension
            "title": "git-livesync",
            "properties": {
                // This is the name of the config setting that we will later interface with through the API
                "git-livesync.enabled": {
                    "type": "boolean",
                    "default": false,
                    "scope": "resource",
                    "description": "CAUTION: DO NOT ENABLE THIS FOR THE USER! Specifies whether to enable this extension in this workspace."
                },
                "git-livesync.commitDelay": {
                    "type": "integer",
                    "default": 30,
                    "scope": "resource",
                    "description": "Set the time in seconds since the last commit before the extension will auto commit your changes."
                }
            }
        }
    ]
```

I didn't mention it in the previous section, but I also wrote a small `src/utils.ts` for holding global variables and functions. I make use of it now to look up the config settings and also initialise a reference to them upon the extension starting up:

```typescript
let commitDelay: number;

export function getCommitDelay(): number {
  return commitDelay; // This lets us call getCommitDelay() from anywhere else in the code.
}

export function initializeCommitDelay() {
  // We call this in the activate function to make sure the first time getCommitDelay() is called, it has a value.
  const configuredCommitDelay = vscode.workspace
    .getConfiguration("git-livesync")
    .get<number>("commitDelay");
  if (typeof configuredCommitDelay === "number") {
    // We implement error checking here in case we fucked up our package.json.
    // Ordinarily, though, there is no way a number could be here.
    commitDelay = configuredCommitDelay;
  }

  // This subscribes to the VS Code API's event system, updating the value of commitDelay whenever the config changes.
  vscode.workspace.onDidChangeConfiguration((event) => {
    if (event.affectsConfiguration("git-livesync.commitDelay")) {
      const newDelay = vscode.workspace
        .getConfiguration("git-livesync")
        .get<number>("commitDelay");
      if (typeof newDelay === "number") {
        commitDelay = newDelay;
      }
    }
  });
}

// git-livesync.enabled follows the exact same code as above
```

_In hindsight, a better implementation would have been looking up the config setting everytime our getter functions were called instead of having it subscribe to the configuration event._

Now we can call the functions and retrieve our config settings, we can use them. Here's an excerpt from `watcher.ts` that shows how we use the enabled config setting to only call the commit function when the config setting is enabled:

```typescript
if (!this.ignoreHandler.ignores(relativeFilePath)) {
  if (getEnabled()) {
    // Calling the function from utils.ts
    this.gitHandler.commit(relativeFilePath, action);
  }
}
```

_The if statements are split up here so I can add debug messages if a file is supposed to be committed but the extension is disabled. I have removed the debug statements from the code snippets in this article, but you can feel free to dig through them in the source repo._

The way I chose to implement the original cooldown was unfortunately pretty shoddy. In essence, it is:

```typescript
// If we can get the last commit's timestamp (through git log) and it was longer ago than the configured commit delay...
if (!(lastCommitTimestamp && now - lastCommitTimestamp < getCommitDelay())) {
  // ...run the commit commands in the terminal.
}
```

_Again, the weird not operator in this if statement is due to me having debug messages._

This initial attempt at a debounce mechanism was not great. The keen eyed among you will spot the immediate problem: some file changes get skipped completely. To explain this, imagine if you were using the extension in this current state. You would start editing a file, type one character that initiates a commit, and the next few characters you typed would be skipped while the delay resolves. This is not a problem on shorter delays, but given a longer configured delay or a slower (in terms of network speed and compute) machine, this could potentially lead to some work being lost in the worst case, or even possible merge conflicts if you are using the extension on multiple machines. Luckily, I do fix this later, so keep reading.

### simple-gitting

I obviously wasn't very happy about this implementation, because the next four hours and five commits (minus documentation or chores) were all about trying to fix this. I'm not used to working with node, and generally try and implement everything from the ground up like an idiot low level programmer, so I had no idea about node's wonderful library of packages that can do everything I wanted better and faster. Introducing [simple-git](https://www.npmjs.com/package/simple-git)! Instead of me trying to run git commands in the VS Code terminal or using node's child process exec, simple-git lets me literally just call git.pull() and everything works automagically.

Anyway, here's what the refactor looked like in `gitHandler.ts`:

```typescript
// I initialise the simpleGit object in the class constructor and give it some parameters.
constructor(context: vscode.ExtensionContext) {
  this.context = context;
  this.inProgress = false;
  const options: Partial<SimpleGitOptions> = {
    baseDir: getWorkspacePath(), // So it will use the current workspace (as is the point of the extension)
    binary: "git", // So it will use all your sneaky little configs
    maxConcurrentProcesses: 6, // This part was in the example so I left it in
    trimmed: false, // Trims whitespace from the commits. Completely unnecessary but also it was in the example
  };
  this.git = simpleGit(options);
}

// Here is the function to commit and sync to the remote. It is now approximately 435% prettier.
public async commit() {
  // Here's a much more elegant debouncing solution.
  // Only one commit process runs at a time (which makes me leaving in maxConcurrentProcesses even dumber) and...
  if (this.inProgress) { return; }
  this.inProgress = true;

  const lastCommitTimestamp = await this.getLastCommitTimestamp();
  const now = Math.floor(Date.now() / 1000);
  const timeSinceLastCommit = now - lastCommitTimestamp;
  const timeDelta = getCommitDelay() - timeSinceLastCommit;
  const sleepDuration = Math.max(0, timeDelta);
  await sleep(1000 * sleepDuration); // ...we just queue up the last commit so it commits all the changes after the delay.

  const commitMessage = new Date().toLocaleString();
  await this.git.pull().add(".").commit(commitMessage).push(); // Doesn't this look a million times better
  this.inProgress = false;
}

private async getLastCommitTimestamp(): Promise<number> {
  // git log -1 --format=%ct returns the UNIX timestamp of the latest commit.
  const log = await this.git.log(["-1", "--format=%ct"]);
  return log.latest
    ? parseInt(log.latest.hash, 10) // It took me like 30 minutes to figure out that log.latest.date is not the date, but instead log.latest.hash
    : Math.floor(Date.now() / 1000);
}
```

Joking aside, I liked learning about how await and async works in Typescript. The concept of Promises, while not an alien one due to me using callbacks in Unity with C#, is a pretty cool one. It's a much more elegant way (IMO) to handle callbacks than all the other implementations I have seen.

Anyway, with my gitHandler class being refactored to be #sexy, we can review what we've achieved so far: an extension that watches for file changes, commits and syncs those changes to a git remote, waits to commit until a configured time has passed, and is also able to be enabled in specific workspaces. We have basically achieved all the functionality we originally planned!

### Documentating

I think its a good time to mention that I preceded each commit, whether it be a refactor, feature, or fix, with an update to README.md. I had a known issues and TODO section that I religiously kept updated. I kinda used to hate planning out what I wanted to do before I did it, but I have to say that setting goals for the next commit and limiting myself to them have definitely led to some better habits, and improved productivity probably by 2 or 3x. This project took me only about ~8 hours of work, compared to the rough week I would have spent on it before I started sticking to best practises. Anyway, back to the project:

## Polishing

There was one more thing I wanted to do before I built the .vsix and tested it on any other repos than a burner; making the extension activate on startup rather than how it currently activated, which was by running a command from the command palette. I have no idea why it took this long for me to figure out that this was literally one line of code to be added to my `package.json`:

```json
  "activationEvents": [
    "workspaceContains:**/.git", // just in case
    "onStartupFinished"
  ]
```

, but this "feature" ended up being the quickest one to implement at a whopping 10 minutes. Thank you VS Code for making it as easy as possible for the Cro Magnon devs of the world <3

### Packaging

Anyway, at that was left to do was to package the extension as an installable .vsix file. This is easily accomplished with:

```bash
vsce package
```

The VSIX file is dropped in the root project directory, and the console output helpfully informs you what is packaged, serving as a useful reference for what to exclude using the `.vscodeignore` file, which for me, ended up being:

```text
.vscode/**
.vscode-test/**
src/**
node_modules
out
webpack.config.js
esbuild.js
.gitignore
.yarnrc
vsc-extension-quickstart.md
**/tsconfig.json
**/eslint.config.mjs
**/*.map
**/*.ts
**/.vscode-test.*
core
not
output*.*
```

It is messy because I just appended exclude patterns to whatever the yeoman generator gave me to start with.

Along with this, I also (following the [tutorial](https://code.visualstudio.com/api/working-with-extensions/bundling-extension)) bundled my extension, which reduced the size of the packaged VSIX file from 10.9MB to 43.63KB (a 2498% size reduction!).

Excited and quivering at finally being able to release a finished piece of software for the first time, I wrote a CI pipeline with Github Actions (which promptly [failed](https://github.com/bwfiq/git-livesync/actions/runs/13183413038/job/36799643138)) and tagged my latest commit with v0.0.1.

### Releasing

Actually, that was a lie because I had to fuck with git for an hour before I figured out how to use release branches and SemVer tagging. This is how I ended up doing it (thank you LLMs):

```bash
# Begin on the latest commit on your main branch
git checkout -b release/v0.0.1
# Do whatever changes you need to make for your release (I removed debug messages and updated the readme)
git commit
git tag v0.0.1 -m "Release v0.0.1" # Tags the latest commit
git push origin v0.0.1 # Or alternatively git push --tags if you have nothing to commit
```

Now that I officially had a commit up on my GitHub remote with the v0.0.1 tag, all I had to do was write a quick changelog (thank you LLMs) and publish it as a release, adding my VSIX file as a binary.

You can find [the release here](https://github.com/bwfiq/git-livesync/releases/tag/v0.0.1)!

### Cleanup

Before we end, I'll just cover what else I did with the project before abandoning it for the foreseeable future.

```bash
git checkout main
git merge release/v0.0.1
git branch -d release/vX.Y.Z
```

This cleaned up my repository by getting rid of the release branch, merging the commits into my main.

## Conclusion

As sardonic as my tone is in this post, I really enjoyed working on this project. First of all, it's my first actual piece of software that I have released, which feels grand. Also, it helps me feel more like a developer now that I have gone through the rite of passage of being annoyed at something and being actually able to code it away.

It's a shame I couldn't just use GitDoc, but I don't regret it. I learned a new language and understand the VS Code extension framework, and by proxy Node and NPM much better. I'm also pretty proud of the entire project only taking two days and for how focused I was on implementing the core functionality and not getting sidetracked as usual, either by irrelevant features, avoidable bugs, or other projects.

The source code for this project is up at <https://github.com/bwfiq/git-livesync/> under a GNU GPL-3.0 license, so feel free to peruse it. All due credit to GitDoc for inspiring this project, and I hope the maintainers figure out how to fix the problem with Foam. Heck, I might try my hand at contributing to an open-source project other than my own for once.

Anyway, I'm planning to write a blog post every week this year and work on something either homelab or code-related every day (minus when I'm not at my desk and touching grass instead). You can follow my exploits on [Github](https://github.com/bwfiq/). See you guys next week where I will be using the dreaded Python snake
