# qButton Pi

[![crates.io page](https://img.shields.io/crates/v/qbutton-pi.svg)](https://crates.io/crates/qbutton-pi)

qButton Pi is a version of qButton which runs on a Raspberry Pi. It uses an attached CC1101 radio
module to listen to 433 MHz RF button codes, and maps them to commands to send to Google Assistant
via its API.

For example, pushing one button could be configured to send the command "bedroom lights on" and
pushing another could send "bedroom lights off".

## Installation

The recommended way to install qButton Pi is from the Debian package. The latest release can be
found on the [GitHub releases page](https://github.com/qwandor/qbutton-pi/releases).

You can also build it yourself with `cargo deb`. In the root of this repository:

```sh
$ cargo install cargo-deb
$ cargo deb
$ dpkg -i target/debian/qbutton-pi_*.deb
```

## Usage

Edit `/etc/qbutton-pi.toml` to fill in your Google Assistant credentials, and then add commands to
the list. See logs for unhandled button IDs. You'll need to restart the `qbutton-pi` service after
editing the config for it to take effect.

## Disclaimer

This is not an officially supported Google product.

## License

Licensed under the Apache License, Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.
