
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use std::io::Write;

fn main() {
  let args = env::args().collect::<Vec<String>>();
  if args.len() != 3 {
    println!("Usage: {} <inputfile>", args[0]); 
  } else {
    let fname_share1 = args[1].trim();
    let fname_share2 = args[2].trim();
    let path1 = Path::new(fname_share1);
    let path2 = Path::new(fname_share2);
    let msg_share1 = File::open(&path1); // return Result<File>
    let msg_share2 = File::open(&path2);

    match (msg_share1, msg_share2) {
      (Ok(msg_share1_f), Ok(msg_share2_f)) => {
        let mut msg_share1_f = msg_share1_f;
        let mut msg_share2_f = msg_share2_f;
        let msg_file = File::create(Path::new("msg"));
        let mut msg_share1_bytes: Vec<u8> = Vec::new();
        let mut msg_share2_bytes: Vec<u8> = Vec::new();
        match msg_share1_f.read_to_end(&mut msg_share1_bytes) {
          Ok(_) => println!("share1 read ok"),
          Err(_) => panic!("share1 read fail!"),
        }
        match msg_share2_f.read_to_end(&mut msg_share2_bytes) {
          Ok(_) => println!("share2 read ok"),
          Err(_) => panic!("share2 read fail!"),
        }
        match msg_file {
          Ok(msg) => join(msg, msg_share1_bytes, msg_share2_bytes),
          Err(_) => panic!("Error opening output file"),
        }
      }
      (_, _) => panic!("Error opening input file"),
    }
  }
}

fn xor(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
  let mut ret = Vec::new();
  for i in 0..a.len() {
    ret.push(a[i] ^ b[i]);
  }
  ret
}

fn join(mut joined_file: File, src_f1: Vec<u8>, src_f2: Vec<u8>) {
  let plain_bytes = xor(src_f1, src_f2);
  match joined_file.write(&plain_bytes[..]) {
    Ok(_) => println!("successfully join file1 and file2"),
    Err(_) => println!("join failed!"),
  };
}