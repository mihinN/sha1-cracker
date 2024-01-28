use std::io;
use std::env;

fn main() {
    print!("Cracker - sha1 - simple simple \n");
    let args_buffer : Vec<String> = env::args().collect();
    dbg!(args_buffer);
}
