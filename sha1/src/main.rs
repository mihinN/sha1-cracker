/********************************
 * PROJECT : SIMPLE SHA1 CRACKER 
 * DATE : 28TH JANUARY 2023
 ********************************
 ******************************** 
*/

use std::{env::{self, args}, error::Error, fs::File , io::{BufRead, BufReader}, };
use sha1::Digest;

const SHA1_HEX_STRING_LENGTH : usize = 40;
fn main() -> Result<(), Box<dyn Error>>{

    print!("Sha1-Decryptor\n");
    println!("===============");

        let args_buffer : Vec<String> = env::args().collect();

        // dbg!(args_buffer);

            if args_buffer.len() != 3 {
                            print!("Usage \n");
                            print!("Decryptor - : <word list> <sha_hash> \n");
                            return Ok(());
            }

        let hash = args_buffer[2].trim();

            if hash.len() != SHA1_HEX_STRING_LENGTH {
                    return Err("Enter a valid sha1 hash".into())
            }

        let word_list = File::open(&args_buffer[1])?;
        let reader = BufReader::new(&word_list);

            for line in reader.lines() {
                        let line = line?;
                        let common_password = line.trim();
                            if hash == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
                                    print!("Password Found : {}\n", common_password);
                                    return Ok(())
                            }
                        print!("{}\n", line);
            }
        
        println!(" Password NOT FOUND ");

    Ok(())
}
