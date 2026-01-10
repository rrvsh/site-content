# Running Containers

The typical flow to run containers on MacOS is to install `colima` and the `docker` CLI.

`colima start` will start a Linux VM via Lima with Docker Engine (by default) inside it, as well as point the Docker CLI to its `dockerd`.
Docker Engine provides the Docker API via `dockerd` and manages container lifecycles with `containerd`.

`dockerd` delegates to `containerd` to:
- Manage container state (created, running, stopped)
- Pull container images
- Unpack the images into a root filesystem snapshot
- Generate an OCI runtime bundle using the snapshot and a `config.json`
- Invokes `runc` to read the bundle and execute the container process
