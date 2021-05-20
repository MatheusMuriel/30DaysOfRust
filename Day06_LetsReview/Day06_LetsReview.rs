use std::io;

fn main() {
  let mut _t_string = String::new();

  io::stdin().read_line(&mut _t_string).ok().expect("Error on read T.");

  let t : i32 = _t_string.trim().parse().ok().expect("Error on parse T.");
  let mut s_list = Vec::new();

  for _ in 0..t {
    let mut _s = String::new();
    io::stdin().read_line(&mut _s).ok().expect("Error on read S.");
    let len_withoutcrlf = _s.trim_end().len();
    _s.truncate(len_withoutcrlf);
    s_list.push(_s);
  }

  for s in s_list {
    let mut even_letters = Vec::new();
    let mut odd_letters = Vec::new();

    for (i, c) in s.chars().enumerate() {
      if (i%2) == 0 {
        even_letters.push(c.to_string());
      } else {
        odd_letters.push(c.to_string());
      }
    }
    
    println!("{} {}", even_letters.join(""), odd_letters.join(""));
  }
}