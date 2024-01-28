/********************************
 * PROJECT : SIMPLE SHA1 CRACKER 
 * DATE : 28TH JANUARY 2023
 ********************************
 ******************************** 
*/

use std::io;
use std::env;

fn main() {

    print!("Sha1-Decryptor\n");
    println!("===============");

    let args_buffer : Vec<String> = env::args().collect();
    // dbg!(args_buffer);
        if args_buffer.len() != 3 {
            print!("Usage \n");
            print!("Decryptor - : <word list> <sha_hash> \n");
            return;
        }
}
