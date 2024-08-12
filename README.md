# Rotel Control

This is a command-line tool to control one's Rotel A12/A14 amplifier via the
integrated ethernet. This tool was created in order to control the amplifier in
a different room using keyboard hotkeys. Whenever I press a specific
combination, KDE starts the tool with `--up` or `--down` and the amplifier is
changed accordingly.

## Compiling

You need to have a working [Rust](https://www.rust-lang.org/) toolchain
installed.

```
git clone https://github.com/maximaximal/rotelcontrol.git
cargo build --release
```

The tool then is in `target/release/rotelcontrol`.

Please use `rotelcontrol --help` to check available options. The Rotel API is
attached and may also be consulted to implement new commands.
