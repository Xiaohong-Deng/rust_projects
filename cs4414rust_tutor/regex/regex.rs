extern crate regex;
use regex::Regex;
use std::str;

fn main() {
  let txt = "GET /favicon.ico HTTP/1.1
    Host: localhost:4414
    User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:46.0) Gecko/20100101 Firefox/46.0
    Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
    Accept-Language: en-US,en;q=0.5
    Accept-Encoding: gzip, deflate
    Connection: keep-alive
    ";
  let re1 = Regex::new(r"^GET.*\n").unwrap();
  let pos1 = re1.find(txt).unwrap();
  let first_line = &txt[pos1.0..pos1.1];
  let re2 = Regex::new(r"\/.*\s").unwrap();
  let pos2 = re2.find(first_line).unwrap();
  let file_path_str = &first_line[pos2.0..pos2.1];
  println!("{}", file_path_str);
}