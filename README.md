# passwdgen
This crate generate a random password via command line.

## Installation

1. Install **Rust**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone this repository
```bash
git clone https://github.com/aldenpower/passwdgen
```

3. Enter in the repository folder and build the project
```bash
cargo build --release
```

4. Copy the binary to your system
```bash
cp target/release/passwdgen /usr/bin/
````

5. Run the binary and enjoy!

```bash
passwdgen
```

## Command-line options

Type the command for help
```bash
passwdgen -h
```
Options:
```bash
    -c, --c <C>          Exclude similar characters [default: true]
    -h, --help           Print help information
    -k, --k <K>          Spaces [default: false]
    -l, --lcl <LCL>      Lower case letters [default: true]
    -n, --n <N>          Numbers [default: true]
    -s, --size <SIZE>    Password lenght [default: 5]
    -t, --t <T>          Strict [default: false]
    -u, --ucl <UCL>      Upper case [default: true]
    -V, --version        Print version information
    -y, --y <Y>          Symbols [default: false]
```

Example:
```bash
passwdgen -s 12 -c false -u false
```