# Running the shell
To run the program, you must compile it: `rustc main.rs`
Then you can rust it using `./main`

This projects aims to implement a very basic shell capable of:
- Executing various basic built in commands: echo; exit; type; pwd; cd;
- Running external programs using the PATH environment variable
- Handling quotes following the [Bash Reference Manual](https://www.gnu.org/software/bash/manual/bash.html#Quoting)
