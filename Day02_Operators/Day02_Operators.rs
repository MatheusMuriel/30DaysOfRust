use std::io;

fn solve(meal_cost : f64, tip_tax : i32, tax_percent : i32) {
  let tip_cost : f64 = (meal_cost / 100f64) * tip_tax as f64;
  let tax_cost : f64 = (meal_cost / 100f64) * tax_percent as f64;
  let total_cost = meal_cost + tip_cost + tax_cost;
  let output : i32 = total_cost.round() as i32;
  println!("{}", output);
}

fn main() {
  let mut _cost_string = String::new();
  let mut _tip_string = String::new();
  let mut _tax_string = String::new();

  io::stdin().read_line(&mut _cost_string).ok().expect("Error on cost read.");
  io::stdin().read_line(&mut _tip_string).ok().expect("Error on tip read.");
  io::stdin().read_line(&mut _tax_string).ok().expect("Error on tax read.");

  let meal_cost   : f64 = _cost_string.trim().parse().ok().expect("Error on cost parse.");
  let tip_percent : i32 = _tip_string.trim().parse().ok().expect("Error on tip parse.");
  let tax_percent : i32 = _tax_string.trim().parse().ok().expect("Error on tax parse.");

  solve(meal_cost, tip_percent, tax_percent);
}