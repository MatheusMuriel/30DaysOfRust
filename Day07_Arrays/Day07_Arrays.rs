use std::io;

fn main() {
  let mut _size_string = String::new();
  let mut _line_string = String::new();
  io::stdin().read_line(&mut _size_string).ok().expect("Error on read size.");
  io::stdin().read_line(&mut _line_string).ok().expect("Error on read line.");

  let mut nums : Vec<String> = _line_string.split(' ').map(|s| s.to_string()).collect();
  nums.reverse();
  let output = nums.join(" ").replace('\n', "");
  println!("{}", output);
}