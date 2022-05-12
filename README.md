# Rustygrep

"A command line tool that interacts with file and command line input/output"

## About The Project

Rust’s speed, safety, single binary output, and cross-platform support make it an ideal language for creating command-line tools, this project is my own version of the classic command-line tool `grep` (globally search a regular expression and print). Implemented only the simplest use case, i.e., it takes a filename and a string as its arguments. Then it reads the file, finds lines in that file that contain the string argument, and prints those lines.

## Built With

- [Rust](https://www.rust-lang.org/)
- [Cargo](https://doc.rust-lang.org/cargo/)

## Getting Started

To get a local copy up and running follow these simple steps:

- Clone or download this repo
```sh
git clone https://github.com/PervezSH/rustygrep.git
```
- Change the current directory to the working directory
```sh
cd token-vesting
```
- To build, use cargo build
```sh
cargo build
```

## License
Distributed under the MIT License. See `LICENSE.txt` for more information.