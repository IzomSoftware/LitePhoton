# LitePhoton: a blazing fast text file reader/scanner/deduplicator


## Overview

- this project is designed to be a blazing fast text file scanner/reader.

### Key Features 🚀

- This project is intended to be lightweight and fast as possible.
- it may not contain many features as of now, but ideas are welcomed.

## Installation

To use LitePhoton, you'll need to have Rust installed on your system. You can download and install Rust from the official website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can clone the LitePhoton repository and build the project:

```
git clone https://github.com/IzomSoftware/LitePhoton.git
cd LitePhoton
cargo build --release
```

The compiled binary will be located in the `target/release` directory.

You can also download binaries available on Github page, without requiring you to do all these.

## Getting Started 🚧

LitePhoton is a command-line tool that can be used to print out contents of a file, or search for a specific keyword in a file. To use it, run the following command:

```
./LitePhoton -f <file_path>
```
the command would print out file contents to stdout (Standard Output).

You can also specify the search method using the `-m` option. The available methods are `simple`, `chunk` and `split`. by default, LitePhoton uses 'split' method.
You can also specify the provider by `-p` option. rayon provides concurrency by rayon library, stdthread provides concurrency by Rust's standard library. by default, LitePhoton uses the 'rayon' provider.

The simple method provides basic search without any kind of concurrency. that being said, specifying a provider with simple method will be ignored.

note that, you can pipe input into LitePhoton. An example could be the cat command.

Example with searching a file for a specific keyword:
```
./LitePhoton -f <file_path> -k <keyword>
```

Example with searching a file for a specific regex:
```
./LitePhoton -f <file_path> -r <regex>
```

Example with deduplicating a file:
```
./LitePhoton -f <file_path> --dedup
```

Example with searching stdin (Standard Input) (Piped or whatever) for a keyword:
```
cat file.txt | ./LitePhoton -k <keyword>
```

Example with searching stdin (Standard Input) (Piped or whatever) with regex:
```
cat file.txt | ./LitePhoton -r <regex>
```

Example with changing the method:
```
./LitePhoton -f <file> -m simple -k <keyword>
```

Other arguments are:
```
--debug -> to enable logging
--dedup -> to dedup a file
--stable -> to provide the stable functionality
```
## A brief comparison
# Test details
- Debian 13
- Intel(R) Xeon(R) E5-2667 v2 (6) @ 3.29 GHz
- 16 GB
<img src="https://media.discordapp.net/attachments/1406334294875570219/1410670184573960262/zaJcqyR.png?ex=68b1dc7c&is=68b08afc&hm=7c52eb368574175fff7186e9fea819f3a3636f33d4a9ed2798b45a81c1c0787a&=&format=webp&quality=lossless&width=1314&height=681"/>


## Contribution Guidelines 🤝
Feel free to contribute to the development of our project. we will notice it.
