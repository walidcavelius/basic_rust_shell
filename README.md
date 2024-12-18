# Basic Rust Shell

## Running the shell
To run the program, you must compile it: `rustc main.rs`
Then you can rust it using `./main`

## About the project
This projects aims to implement a very basic [POSIX](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html) compliant shell capable of:
- Executing various basic built in commands: `echo`, `exit`, `type`, `pwd`, `cd`
- Running external programs using the `PATH` environment variable
- Handling quotes following the [Bash Reference Manual](https://www.gnu.org/software/bash/manual/bash.html#Quoting)

This was done in an attempt to learn both about the underlying concepts behind shells and about Rust
