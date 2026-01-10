# Cargo Workspaces

Define a workspace with a [virtual manifest](https://doc.rust-lang.org/cargo/reference/workspaces.html#virtual-workspace) by initialising a `Cargo.toml` with the following content:

```toml
[workspace]
resolver = "3"
```

The resolver key determines how Cargo resolves package versions in the case of e.g. multiple dependencies relying on the same crate with different versions.

Add member crates to the workspace by running `cargo add` in the same directory as the workspace `Cargo.toml`. This will add the newly created package to the `members` key in the workspace `Cargo.toml`:

```toml
[workspace]
resolver = "3"
members = ["new-crate"]
```

## References
- https://doc.rust-lang.org/cargo/reference/resolver.html
- https://doc.rust-lang.org/cargo/reference/workspaces.html
