fn main() {
  let body = "GET /favicon.ico HTTP/1.1
    Host: localhost:4414
    User-Agent: Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:46.0) Gecko/20100101 Firefox/46.0
    Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
    Accept-Language: en-US,en;q=0.5
    Accept-Encoding: gzip, deflate
    Connection: keep-alive
    ";
  let start = body.find(char::is_whitespace).unwrap() + 1;
  let end = (&body[start..]).find(char::is_whitespace).unwrap() + 4;
  let file_path_str = &body[start..end];
  println!("{}", file_path_str);
}