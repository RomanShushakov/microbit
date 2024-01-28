## Commands used

- `minicom -D /dev/ttyACM0 -b 115200`

- `cargo build --target thumbv7em-none-eabihf`
- `cargo embed --target thumbv7em-none-eabihf`

- `cargo build --target thumbv7em-none-eabihf --release`
- `cargo embed --target thumbv7em-none-eabihf --release`

- `cargo size --target thumbv7em-none-eabihf -- -A`
- `cargo size --target thumbv7em-none-eabihf --release -- -A`
