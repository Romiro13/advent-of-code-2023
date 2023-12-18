use std::fs;

mod day_01;
mod day_02;
mod day_03;
mod day_04;

#[derive(PartialEq)]
pub enum Puzzle {
  Part01,
  Part02,
}

fn main() -> anyhow::Result<()> {
  let input = fs::read_to_string("inputs/input_day01.txt")?;

  let rs = day_01::get_sum_calibration_value(&input, &Puzzle::Part01);
  println!("Day 01 Part01 result is {}", rs);

  let rs = day_01::get_sum_calibration_value(&input, &Puzzle::Part02);
  println!("Day 01 Part02 result is {}", rs);

  let input = fs::read_to_string("inputs/input_day02.txt")?;
  let rs = day_02::play(&input, &Puzzle::Part01);
  println!("Day 02 Part01 result is {}", rs);

  let rs = day_02::play(&input, &Puzzle::Part02);
  println!("Day 02 Part02 result is {}", rs);

  let input = fs::read_to_string("inputs/input_day03.txt")?;
  let rs = day_03::play(&input, &Puzzle::Part01);
  println!("Day 03 Part01 result is {}", rs);

  let rs = day_03::play(&input, &Puzzle::Part02);
  println!("Day 03 Part02 result is {}", rs);

  let input = fs::read_to_string("inputs/input_day04.txt")?;
  let rs = day_04::play(&input, &Puzzle::Part01);
  println!("Day 04 Part01 result is {}", rs);

  let rs = day_04::play(&input, &Puzzle::Part02);
  println!("Day 04 Part02 result is {}", rs);

  Ok(())
}
