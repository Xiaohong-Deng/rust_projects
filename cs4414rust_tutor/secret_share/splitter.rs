use rand::Rng;
use std::env;
use std::fs::File;

fn main() {
    let args: &str = (env::args().collect::<Vec<String>>());
    if args.len() != 2 {
        println!("Usage: {:s} <inputfile>", args[0]); 
    } else {
        let fname = args[1];
        // Path::new() take as argument a string slice
        // return a Path slice which is an unsized type 
        // must be used behind a pointer like & or Box
        let path = Path::new(fname.clone());
        // msg_file is Result<File>
        let msg_file = File::open(&path);

        match (msg_file) {
            Ok(mut msg) => {
                let mut msg_bytes: Vec<u8> = Vec::new();
                try!(msg.read_to_end(&mut msg_bytes));
                let share1_file 
                       = File::create(&Path::new(fname + ".share1"));
                let share2_file 
                       = File::create(&Path::new(fname + ".share2"));
                
                match (share1_file, share2_file) {
                    (Ok(share1), Ok(share2)) => { 
                        split(msg_bytes, share1, share2); 
                        } ,
                    (_, _) => panic!("Error opening output files!"),
                }
            } ,
            Err(_) => panic!("Error opening message file: {:s}", fname)
        }
    }
}

fn xor(a: &Vec<u8>, b: &Vec<u8>) -> mut Vec<u8> {
    let mut ret = Vec::new();
    for i in range(0, a.len()) {
      ret.push(a[i] ^ b[i]);
    }
    ret
}

fn split(msg_bytes: &[u8], mut share1: File, mut share2: File) {
    let mut random_bytes: &[u8] = &[];
    // This is not cryptographically strong randomness! 
    // (For entertainment purposes only.)
    for _ in 0..msg_bytes.len() {
      let random_byte = random(); // random_byte is a Vec
      random_bytes.push(random_byte);
    }
    
    let encrypted_bytes = xor(msg_bytes, random_bytes);
    share1.write(random_bytes);
    share2.write(encrypted_bytes);
}