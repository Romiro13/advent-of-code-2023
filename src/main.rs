use std::fs;

mod day_01;
mod day_02;

fn main() -> anyhow::Result<()> {
  let input = fs::read_to_string("inputs/input_day01.txt")?;
  let rs = day_01::get_sum_calibration_value(input);
  println!("result day 01: {}", rs);

  Ok(())
}
