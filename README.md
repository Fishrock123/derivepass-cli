[![Crates.io](https://img.shields.io/crates/v/derivepass-cli.svg)](https://crates.io/crates/derivepass-cli)

# derivepass-cli

Simple key derivation utility.

A command line version of https://derivepass.com/.

## Installation

```sh
cargo install derivepass-cli
```

## Usage

Specify a domain and username with arguments, then enter a master password in order to derive a unique, constant key that can be recomputed at any time with the same information.

```
derivepass-cli -d <domain.com> -u <myusername>
```

### Options

- `-d`, `--domain`: The domain (website domain or service name).
- `-u`, `--user`: Your username for your account on the domain.
- `-r`, `--revision`: Password revision (use this if you need to change the password).

## License

[BlueOak-1.0.0 Licensed](LICENSE) — _[Contributions via DCO 1.1](contributing.md#developers-certificate-of-origin)_
