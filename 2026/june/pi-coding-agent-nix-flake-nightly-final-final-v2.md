---
title: pi-coding-agent-nix-flake-nightly-final-final-v2
slug: pi-coding-agent-nix-flake-nightly-final-final-v2
date: 2026-06-25
tags: [daily-blog, nix, github-actions]
---

I just wrote a small flake for packaging [pi-coding-agent](https://github.com/earendil-works/pi), mostly so I can install the coding agent through Nix on my machines instead of managing it separately with npm.

Originally, I was just using the `nixpkgs` package. This worked fine, but Pi moves quickly and `nixpkgs` takes a while to update. I was running into several bugs that were fixed upstream and wanted something a bit closer to latest without having to wait.

I tried [lukasl-dev/pi.nix](https://github.com/lukasl-dev/pi.nix), which is an existing flake for Pi. It was useful, but it was not quite what I wanted. It uses its own custom package definitions, one of which builds with Bun, and I kept running into breakage on my Darwin system. I do not really need anything fancy here; I just want the Pi package as close to the `nixpkgs` package as possible, with the version and hashes updated automatically.

I also do not expect official Nix support to land upstream any time soon. In [earendil-works/pi#2310](https://github.com/earendil-works/pi/issues/2310), Mario Zechner said:

> sorry, not a nix user, no idea what it does, how it works, and no time to maintain this.

Which is completely fair, but it also means flakes are probably not coming to Pi itself.

## Making the package updateable

Before writing the updater, I first had to slightly adapt the vendored package definition.

The original `nixpkgs` package is a normal package file where things like `buildNpmPackage`, `fetchFromGitHub`, `ripgrep`, `fd`, and `makeBinaryWrapper` are passed in as arguments. My repo uses `flake-parts`, so the file is instead a small `flake-parts` module using `perSystem`.

That meant changing those references to come from `pkgs` instead:

```nix
perSystem =
  { pkgs, ... }:
  {
    packages.pi-coding-agent = pkgs.buildNpmPackage {
      # ...
    };
  };
```

so `buildNpmPackage` became `pkgs.buildNpmPackage`, `fetchFromGitHub` became `pkgs.fetchFromGitHub`, and so on.

The other change was moving the release-specific values out of the package definition. Instead of hardcoding the version and hashes directly in the Nix file, I read them from `VERSION.json` at evaluation time:

```nix
let
  cfg = config.flake;
  inherit (cfg.paths) root;
  versionInfo = builtins.fromJSON (builtins.readFile (root + /VERSION.json));
in
{
  # ...
}
```

Then the package uses those values:

```nix
version = versionInfo.version;

src = pkgs.fetchFromGitHub {
  owner = "earendil-works";
  repo = "pi";
  tag = "v${versionInfo.version}";
  hash = versionInfo.srcHash;
};

npmDepsHash = versionInfo.npmDepsHash;
```

This means the actual build logic can stay basically the same as the `nixpkgs` package, while the parts that need to change on every release are all isolated in one JSON file that can then be updated independently.

For this package, updating means changing three values:

```json
{
  "version": "0.80.2",
  "srcHash": "sha256-aKtgPc3rwHEp856jP3N7nImph0CSG+gsWq9OVci3hmE=",
  "npmDepsHash": "sha256-1EGs8lX8XoAnRtS+pw4lBRm24U/vtVB2loVRmZyd4Z8="
}
```

The `version` is just the upstream Pi release version, without the leading `v`.

The `srcHash` is the hash of the source archive fetched from GitHub. Nix needs this because fetchers are fixed-output derivations. It will only accept the downloaded source if it matches the hash declared in the package.

The `npmDepsHash` is similar, but for the npm dependency tree. `buildNpmPackage` vendors the dependencies from `package-lock.json`, and the resulting dependency set also has to match a declared hash. If upstream changes its lockfile, or if a new release has different dependencies, this hash needs to be updated too.

## The update script

The next step was writing the actual updater. The agent made this live in the flake itself instead of only in the GitHub Actions workflow, so that the CI job would just run the same command I can run locally:

```bash
nix run .#update
```

The script starts by asking GitHub for the latest Pi release:

```bash
tag=$(curl -fsSL https://api.github.com/repos/earendil-works/pi/releases/latest | jq -r .tag_name)
version="${tag#v}"
archive_url="https://github.com/earendil-works/pi/archive/refs/tags/${tag}.tar.gz"
```

The GitHub releases API returns JSON, so `curl` fetches it and `jq -r .tag_name` extracts the tag name as a raw string. The `-r` matters because without it, `jq` would return the JSON string with quotes around it.

Pi tags look like this:

```text
v0.80.2
```

but the package version should be:

```text
0.80.2
```

so this line strips the leading `v`:

```bash
version="${tag#v}"
```

This is Bash parameter expansion. `${tag#v}` removes the shortest matching `v` prefix from `$tag`.

Next, the script computes the source hash:

```bash
src_hash=$(nix store prefetch-file --json --unpack "$archive_url" | jq -r .hash)
```

`nix store prefetch-file` downloads the file and tells us the hash Nix expects for it. The `--json` flag makes the output easy to parse with `jq`.

The `--unpack` flag is there because `pkgs.fetchFromGitHub` fetches and unpacks the archive. The hash in the package needs to match the unpacked source tree, not just the raw tarball.

After that, the script needs the npm dependency hash. For that, it downloads the source into a temporary directory:

```bash
workdir=$(mktemp -d)
trap 'rm -rf "$workdir"' EXIT
```

`mktemp -d` creates a temporary directory, and the `trap` makes sure it gets cleaned up when the script exits. This runs whether the script succeeds or fails, which is a nice and simple way to avoid leaving temporary source trees around.

The npm hash itself comes from [`prefetch-npm-deps`](https://github.com/NixOS/nixpkgs/blob/nixos-unstable/pkgs/build-support/node/prefetch-npm-deps/default.nix#L201):

```bash
curl -fsSL "$archive_url" | tar -xz -C "$workdir"
src_dir="$workdir/pi-${version}"
npm_hash=$(prefetch-npm-deps "$src_dir/package-lock.json")
```

It reads the upstream `package-lock.json`, fetches the dependencies described by it, and prints the hash that `buildNpmPackage` expects for `npmDepsHash`.

Finally, the script writes the new metadata file and updates the flake's lockfile:

```bash
jq -n \
  --arg version "$version" \
  --arg srcHash "$src_hash" \
  --arg npmDepsHash "$npm_hash" \
  '{version: $version, srcHash: $srcHash, npmDepsHash: $npmDepsHash}' > VERSION.json

nix flake update
```

So the update command refreshes both `VERSION.json` and `flake.lock`.

## GitHub Actions

Once the local updater worked, the next step was using GitHub compute to do it for me.

I started with the build workflow first. The purpose of this workflow is simple: on every push to `main` and every pull request, build the package on both supported systems, `x86_64-linux` and `aarch64-darwin`.

It also configures Cachix (a Nix binary cache) with my cache and auth token. This means successful builds populate `rrvsh.cachix.org`, so after CI has built a version once, my machines can reuse the cached result instead of rebuilding everything locally.

After that, I got the agent to use the `gh` CLI/API to configure the repository settings. The important pieces were:

- enable GitHub auto-merge for the repository
- protect `main` by requiring pull requests
- require the two build jobs before merging

I did not require PR approvals for this, because the update PRs are mechanical. The thing I care about is whether the package builds on Linux and Darwin. If those checks pass, GitHub can merge the PR.

With that in place, the update workflow becomes quite small conceptually. It runs every day, or manually through `workflow_dispatch`, and runs:

```bash
nix run .#update
```

Then it checks whether either of the files managed by the updater changed:

```text
flake.lock
VERSION.json
```

If there are no changes, the workflow exits successfully. If there are changes, it commits them to a branch called:

```text
chore/update-pi-package
```

and opens a pull request back into `main`.

The final step is enabling auto-merge on that PR:

```bash
gh pr merge --auto --delete-branch --squash "$PR_NUMBER"
```

This is the part I like about the setup. The update workflow is not responsible for deciding whether the update is good. It just creates the PR and asks GitHub to merge it when the rules are satisfied. Branch protection still requires the Linux and Darwin builds, so if either package build fails, the PR stays open instead of being merged into `main`. In future I may try and automate remediation for the update failing by plugging in an LLM to push a fix, but for now I am lazy to wire up GHA.

## What did not work right away

The first was GitHub permissions. The first scheduled run failed with:

```text
pull request create failed: GraphQL: GitHub Actions is not permitted to create or approve pull requests (createPullRequest)
```

This was fixed by enabling the repository setting that allows GitHub Actions to create and modify pull requests.

### GPT INTERJECTION: `--force` vs `--force-with-lease`

The following section is written in my own voice, because I cared more than Rafiq did about using `--force-with-lease` correctly.

The update workflow needs to overwrite the same branch repeatedly:

```bash
chore/update-pi-package
```

From a purely practical perspective, `git push --force` would work here. The branch is generated by automation, it is not meant to be a long-lived human development branch, and its previous state usually does not matter. If the updater finds a newer Pi release, the desired outcome is simply for the remote update branch to point at the newest generated commit.

However, `--force-with-lease` is a better default than `--force`, even for this kind of branch.

A normal force push is unconditional. It tells Git to update the remote branch to the local commit regardless of what is currently on the remote. `--force-with-lease` adds a safety check: it only force-pushes if the remote branch still points to the commit that the local repository believes it points to.

In other words:

```text
--force
  Replace the remote branch.

--force-with-lease
  Replace the remote branch only if nobody changed it behind my back.
```

That distinction matters most on shared human branches, but it is still useful for automation. It protects against accidental overwrites from concurrent workflow runs, manual debugging commits, or stale local state after a previous failed job.

The failed version of the workflow used `--force-with-lease`, but did not first fetch the remote update branch. In a fresh GitHub Actions checkout, the local repository may not have a useful `origin/chore/update-pi-package` reference. When Git tried to lease against that missing or stale reference, the push failed with:

```text
! [rejected] chore/update-pi-package -> chore/update-pi-package (stale info)
```

The fix was to fetch the remote branch before pushing:

```bash
git fetch origin "refs/heads/$BRANCH_NAME:refs/remotes/origin/$BRANCH_NAME" || true
git push --force-with-lease origin "$BRANCH_NAME"
```

The fetch updates the local remote-tracking ref, giving `--force-with-lease` something accurate to compare against. The `|| true` handles the first run, where the branch may not exist yet.

This keeps the behaviour the workflow needs—it can replace its own generated update branch—without using a completely blind force push.

### Not updating merged PRs

The last issue was more annoying. I left the repository alone for a few days and checked on it today to find it was wholly not working. Some investigatory work found that the workflows were succeeding but no PRs were being opened with the updates.

My first version of the PR reuse logic used:

```bash
gh pr view "$BRANCH_NAME"
```

I expected this to mean "look for the current open PR for this branch", but that is not quite what it does. After the first update PR was merged, later scheduled runs would still point to the old merged PR for the same branch name. The workflow would then push new commits to `chore/update-pi-package`, decide that the PR already existed, and never open a fresh one.

The fix was to explicitly ask for an open PR:

```bash
PR_NUMBER=$(gh pr list \
  --state open \
  --base main \
  --head "$BRANCH_NAME" \
  --json number \
  --jq '.[0].number // empty')
```

and then pass that PR number to the merge command:

```bash
gh pr merge --auto --delete-branch --squash "$PR_NUMBER"
```

This made the behaviour match what I actually wanted: reuse an existing open update PR if there is one, otherwise create a new PR. After that fix, the updater successfully opened a fresh PR for `0.80.2`, the Linux and Darwin builds passed, and GitHub auto-merged it.

## Conclusion

Did I really need a nightly updated pi-coding-agent on my systems? Nope, but it was still fun to learn how to get a nix package auto updated with GitHub Actions, and now I offload some compute to Microslop. See ya tomorrow!
