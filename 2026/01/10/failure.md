---
slug: failure
tags: [daily-blog]
---
# Failure

So I had a grand vision to set up all the needed infrastructure for my website today, learning GCP and Terraform (technically OpenTofu) along the way - that failed, because after 14 hours of trials and tribulations, it still refuses to work.

Successes despite the above:
- Initialised the Rust crate
- Built it as an OCI container with Nix
- Set up the base OpenTofu stuff

I think what I will be doing next week (because tomorrow is purely for DnD) is going to be merging whatever was of value from the in-progress PR, and discarding the rest. I will leave you with something from this morning - a summary of my research consolidating basically everything I've learnt about working with Docker for the last few years:

## Running Containers

The typical flow to run containers on MacOS is to install `colima` and the `docker` CLI.

`colima start` will start a Linux VM via Lima with Docker Engine (by default) inside it, as well as point the Docker CLI to its `dockerd`.
Docker Engine provides the Docker API via `dockerd` and manages container lifecycles with `containerd`.

`dockerd` delegates to `containerd` to:
- Manage container state (created, running, stopped)
- Pull container images
- Unpack the images into a root filesystem snapshot
- Generate an OCI runtime bundle using the snapshot and a `config.json`
- Invokes `runc` to read the bundle and execute the container process

## Conclusion

See ya tomorrow!
