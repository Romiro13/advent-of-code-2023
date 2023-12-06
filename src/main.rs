use std::fs;

mod day_01;
mod day_02;

fn main() -> anyhow::Result<()> {
  let input = fs::read_to_string("inputs/input_day01.txt")?;

  let rs = day_01::get_sum_calibration_value(&input, &day_01::Puzzle::Part01);
  println!("Day 01 Part01 result is {}", rs);

  let rs = day_01::get_sum_calibration_value(&input, &day_01::Puzzle::Part02);
  println!("Day 01 Part02 result is {}", rs);

  let input = fs::read_to_string("inputs/input_day02.txt")?;
  let rs = day_02::sum_ids_of_game(&input);
  println!("Day 02 Part01 result is {}", rs);

  Ok(())
}
