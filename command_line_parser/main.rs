use std::env::*;

fn main() {
    // Prints each argument on a separate line
    for argument in std::env::args() {
        println!("{}", argument);
    }
}
