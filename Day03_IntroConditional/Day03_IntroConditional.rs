use std::io;

fn main() {
  let mut _n_string = String::new();

  io::stdin().read_line(&mut _n_string).ok().expect("Error on n read.");

  let n : i32 = _n_string.trim().parse().ok().expect("Error on n parce.");

  if n%2 != 0 {
    println!("Weird");
  } else if n >= 2 && n <= 5 {
    println!("Not Weird");
  } else if n >= 6 && n <= 20 {
    println!("Weird");
  } else if n >= 20 {
    println!("Not Weird");
  }
}