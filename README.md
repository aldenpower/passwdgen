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
   OPTIONS:
    -c, --lowercaseletters <lcl>    Contain lower case letters [default: false]
    -h, --help                      Print help information
    -k, --spaces <spaces>           Contain spaces [default: false]
    -n, --numbers <numbers>         Contain numbers [default: false]
    -o, --symbol <symbol>           Contain symbols [default: false]
    -s, --size <size>               Password length [default: 5]
    -t, --similar <similar>         Contain similar characters [default: false]
    -u, --uppercaseletters <ucl>    Contain upper case letters [default: false]
    -V, --version                   Print version information 
```

Example:

A ten-digit password containing symbols, numbers, lower case and upper case letters without repeated characters.

```bash
passwdgen -s 10 -o -n -c -u
```