Cargo LLDB Runner
===

One of probably a hundred Rust implementations to get around SIP in MacOS. This
used to be implemented as a shell script until I suddenly stumbled over a
proc macro test that linked to Rust's libstd. SIP strips off the
DYLD_FALLBACK_LIBRARY_PATH when invoking sh/bash/zsh so here we are.

Usage
---

Build this project and copy the resulting binary somewhere. Then in your
`~/.cargo/config.toml` add somthing to the effect of:

```toml
[target.aarch64-apple-darwin]
runner = ["/path/to/bin/cargo-lldb-runner"]
```

Then if any of your tests segfault you can re-run them in LLDB by prefixing
the Cargo command with `DEBUG=1` or similar:

```zsh
DEBUG=1 cargo test
```
