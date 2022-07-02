# Blinker

Simple GPIO project for Raspberry Pi.

## Cross-compile
I found my Raspberry Pi 3, struggling to compile the code so decided to cross-compile it on my main machine.  

The project is currently set to cross-compile for Raspberry Pi with `aarch64` architecture.
More cross-compile options can be added to the `.cargo/config.toml` file.
Matching compiler and linker need also be installed on your machine.
To compile for selected platform use `cargo build --target=<target-config-name>` e.g. `cargo build --target=aarch64-unknown-linux-gnu`.

<img src="/images/circuit.png" width="400" height="500" alt="Circuit schema" /> <img src="/images/example.jpg" width="400" height="500" alt="Example" />
