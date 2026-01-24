---
title: Automating Submodule Updates with Git Hooks
slug: git-submodule-hooks
date: 2025-02-02
tags: [git]
---

I have a public wiki for my DnD setting up at <https://aenyrathia.wiki/>. It runs on [Otterwiki](https://otterwiki.com/), which means I have access to the entire wiki of markdown files as a git repository. While this is amazing for accessing my wiki from anywhere and having version control, it also means I don't have an easy way to add my own private DM notes.

To this end, I'd like to have my own private repository that pulls in the wiki as a submodule so I can easily reference it for my own notes without having to consult the website. Here's the problem: normally, submodules are locked at a specific commit, which makes sense because they are meant to act as dependencies for the parent repository. The thing is, I'm not working with code in this case. I want the submodule to be constantly up to date and have the latest changes pulled from the wiki.

## Problems

Right now, I can have `git pull` (and the other commands) recursively update the module through a simple:

```bash
git config pull.recursesubmodules true
```

THe issue now is that because of the way submodules work (primarily the fact that they are a specific checked out commit of the other repository), this doesn't pull in the latest changes from the submodule repository. To actually achieve that, I have to run the command:

```bash
git submodule update --remote
```

which pulls in the changes from the remote.

### Drawbacks to this method

We have to acknowledge why this "workaround" is not more widely used. I am using git submodules in a non-standard way here, and this approach would lead to some potential issues, revolving around the fact that if the rest of your code depends on these submodules, pulling in random changes could lead to modifications or errors in the logic of the parent repository's code. However, this doesn't apply to this project, because markdown files aren't gonna blow up when I change other files. If you want to use this method though, do be warned!

## Automation

So, now we have identified the problem and a solution. How do we then prevent having to always run the `git submodule update` command every time we pull?

### Git Hooks

Literally as I was researching potential solutions for this earlier problem, I was messing with my VS Code configs in an attempt to find out why my .git folder wasn't being shown in the explorer pane. I found out through a [blog post](https://medium.com/pareture/show-git-and-other-default-hidden-folders-and-files-in-vs-code-57df151588ea) that these excluded folders were in the settings and promptly removed all the default rules, but the post also mentioned why the author wanted to include the .git folder in their explorer, namely because they were using git hooks.

I had absolutely no idea what these were and skimmed past it since it wasn't related to my issue, but when I was thinking of how I could automate this solution, the git hooks came back to mind as something to look into. Thankfully, I was right in my hunch, and git hooks turned out to be the solution. All I had to do was drop a shell script named `post-merge` into the .git/hooks folder and make it executable, and whatever commands were in that file would be run upon a pull command.

The contents I put in my `.git/hooks/post-merge` script were as follows:

```sh
git submodule update --init --recursive --remote
```

You'll notice the --init and --recursive flags being included which weren't mentioned above, but all they are for is for when you clone my parent repository (which doesn't automatically initialise the submodule repository) and are more for if I forget to also init the submodule when cloning this repo next time.

With that being said, now whenever I pull the repository, the submodule nicely populates itself with whatever changes has been made to the wiki.

Next steps are to see if I can't make this easier to work with by using the [GitDoc](https://marketplace.visualstudio.com/items?itemName=vsls-contrib.gitdoc) extension to be able to work with my wiki like I'm used to in Obsidian with livesync!
