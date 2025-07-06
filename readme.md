
# Cyber Reachability

This tool contains strategies to read data from everything it can access,
beginning with the system it is executed from.

It logs a report of all reachable cyber assets and access levels.
Additional access may be provided with a folder of credentials, one credential
per `.toml` file.

# Cross-Compilation

`cyber-reachability[.exe]` copies itself to discovered hosts upon which we have shell access (ssh, rdp) and
continues scans from there; this enables hopping from network to network enumerating all hosts, even those
unreachable without access credentials.

To enable easy cross-compilation of the tool for use on linux, windows, and macos systems we employ
`cargo-zigbuild` which requires `zig` to be installed.

```bash
cargo install --locked cargo-zigbuild
cargo zigbuild --release --target x86_64-pc-windows-gnu
cargo zigbuild --release --target x86_64-unknown-linux-gnu
cargo zigbuild --release --target x86_64-apple-darwin
```

Currently unsupported platforms for scan hops:

 - aarch64 windows
 - aarch64 linux
 - aarch64 macos


# Credential Types

## SSH

```

```





