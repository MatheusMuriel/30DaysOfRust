use std::io;

fn main() {
  let mut _n_string = String::new();
  io::stdin().read_line(&mut _n_string).ok().expect("Error on read N.");

  let n : i32 = _n_string.trim().parse().ok().expect("Error on parce N.");

  for i in 1..11 {
    println!("{} x {} = {}", n, i, (n*i));
  }
}