use std::env::*;

fn main() {
    // Prints each argument on a separate line
    // The code snippet below would be equivalent to 
    // for(i = 0; i < argc; i++) printf("%s\n", argv[i]) in C language
    for argument in std::env::args() {
        println!("{}", argument);
    }
}
