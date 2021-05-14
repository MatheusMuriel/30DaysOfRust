use std::io;

// https://www.hackerrank.com/challenges/30-data-types/problem
fn main() {
  let i = 4;
  let d = 4.0;
  let s = "HackerRank ";

  let mut _my_integer_str = String::new();
  let mut _my_double_str = String::new();
  let mut _my_string = String::new();

  io::stdin().read_line(&mut _my_integer_str).ok().expect("Read Error on 1º Line!");
  io::stdin().read_line(&mut _my_double_str).ok().expect("Read Error on 2º Line!");
  io::stdin().read_line(&mut _my_string).ok().expect("Read Error on 3º Line!");

  let mut _my_integer : i32 = _my_integer_str.trim().parse().ok().expect("Parse Error on 1º number");
  let mut _my_double : f64 = _my_double_str.trim().parse().ok().expect("Parse error on 2º number");

  println!("{}", i + _my_integer);
  println!("{}", d + _my_double);
  println!("{}{}", s, _my_string);
}