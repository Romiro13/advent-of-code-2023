use itertools::Itertools;
use std::collections::BTreeMap;

use crate::Puzzle;

#[derive(Debug, Eq, Clone)]
struct Point {
  y: i32,
  x: i32,
}

#[derive(Debug, Clone)]
struct Number {
  points: Vec<Point>,
  value: String,
  is_valid: bool,
}

impl Point {
  fn new(y: i32, x: i32) -> Self {
    Self { y, x }
  }
}

impl Ord for Point {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    (self.y, self.x).cmp(&(other.y, other.x))
  }
}

impl PartialOrd for Point {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Point {
  fn eq(&self, other: &Self) -> bool {
    self.y == other.y && self.x == other.x
  }
}

impl Number {
  fn new(points: Vec<Point>, value: String, is_valid: bool) -> Self {
    Self {
      points,
      value,
      is_valid,
    }
  }
}

pub fn play(input: &str, puzzle: &Puzzle) -> i32 {
  if puzzle.eq(&Puzzle::Part01) {
    get_sum_engine_schematic(input)
  } else {
    get_sum_all_gear_rations(input)
  }
}

fn create_positions_to_check(pos: &Point) -> Vec<Point> {
  let pos_check: Vec<Point> = vec![
    Point::new(0, -1),
    Point::new(-1, -1),
    Point::new(1, -1),
    Point::new(-1, 1),
    Point::new(0, 1),
    Point::new(1, 1),
    Point::new(-1, 0),
    Point::new(1, 0),
  ];

  pos_check
    .iter()
    .map(|p| Point::new(p.y + pos.y, p.x + pos.x))
    .collect::<Vec<_>>()
}

fn get_sum_engine_schematic(input: &str) -> i32 {
  let lines = input.trim().lines().collect::<Vec<_>>();

  let mut chars: BTreeMap<Point, char> = BTreeMap::new();

  for (y, line) in lines.iter().enumerate() {
    for (x, chr) in line.trim().chars().enumerate() {
      if chr != '.' {
        chars.insert(Point::new(y as i32, x as i32), chr);
      }
    }
  }

  let mut numbers: Vec<Number> = vec![];
  let mut i = 0;
  for (pos, chr) in chars.iter() {
    if chr.is_ascii_digit() {
      let pos_check = create_positions_to_check(pos);

      let mut is_valid = false;
      for p in pos_check.iter() {
        if let Some(v) = chars.get(p) {
          if !v.is_ascii_digit() {
            is_valid = true;
            break;
          }
        };
      }

      let mut is_new_number = false;

      if let Some(number) = numbers.last_mut() {
        if i + 1 == pos.x {
          number.value.push(*chr);
          if !number.is_valid {
            number.is_valid = is_valid;
          }
        } else {
          is_new_number = true;
        }
      } else {
        is_new_number = true;
      }

      if is_new_number {
        numbers.push(Number::new(vec![], chr.to_string(), is_valid));
      }

      i = pos.x;
    }
  }

  numbers
    .iter()
    .filter(|n| n.is_valid)
    .map(|n| n.value.parse::<i32>().unwrap())
    .sum()
}

fn get_sum_all_gear_rations(input: &str) -> i32 {
  let lines = input.trim().lines().collect::<Vec<_>>();

  let mut chars: BTreeMap<Point, char> = BTreeMap::new();

  for (y, line) in lines.iter().enumerate() {
    for (x, chr) in line.trim().chars().enumerate() {
      if chr != '.' {
        chars.insert(Point::new(y as i32, x as i32), chr);
      }
    }
  }

  let mut numbers: Vec<Number> = vec![];
  let mut all_valid_points: Vec<Vec<Point>> = vec![vec![]];
  let mut gear_rations: Vec<i32> = vec![];
  let mut i = 0;

  for (pos, chr) in chars.iter() {
    if chr.eq(&'*') {
      let pos_check = create_positions_to_check(pos);
      for p in pos_check.iter() {
        if let Some(v) = chars.get(p) {
          if v.is_ascii_digit() {
            all_valid_points.last_mut().unwrap().push(p.clone());
          }
        };
      }
      all_valid_points.push(vec![]);
    };

    if chr.is_ascii_digit() {
      let mut is_new_number = false;

      if let Some(number) = numbers.last_mut() {
        if i + 1 == pos.x {
          number.value.push(*chr);
          number.points.push(pos.clone());
        } else {
          is_new_number = true;
        }
      } else {
        is_new_number = true;
      }

      if is_new_number {
        numbers.push(Number::new(vec![pos.clone()], chr.to_string(), false));
      }

      i = pos.x;
    }
  }

  let valid_pos = all_valid_points
    .into_iter()
    .filter(|v| v.len() >= 2)
    .collect::<Vec<Vec<Point>>>();

  for v in valid_pos {
    let mut valid_numbers: Vec<i32> = vec![];
    for p in v.iter() {
      let n = numbers
        .iter()
        .filter(|n| n.points.contains(p))
        .filter_map(|n| n.value.parse::<i32>().ok())
        .last()
        .unwrap();

      valid_numbers.push(n);
    }

    let valid_numbers = valid_numbers.into_iter().unique().collect::<Vec<i32>>();

    if valid_numbers.len() > 2 {
      panic!("There is no gear ration format for more than two numbers");
    }

    if valid_numbers.len() < 2 {
      continue;
    }
    gear_rations.push(valid_numbers.iter().product());
  }

  gear_rations.iter().sum()
}

// region:    --- Tests
#[cfg(test)]
mod tests {

  use super::*;
  use anyhow::Result;

  #[test]
  fn test_day03_play_part01_ok() -> Result<()> {
    let fx_input = r#"
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598.."#;
    let fx_expected = 4361;

    let rs = play(fx_input, &Puzzle::Part01);

    assert_eq!(rs, fx_expected);

    Ok(())
  }

  #[test]
  fn test_day03_play_part02_ok() -> Result<()> {
    let fx_input = r#"
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598.."#;
    let fx_expected = 467835;

    let rs = play(fx_input, &Puzzle::Part02);

    assert_eq!(rs, fx_expected);

    Ok(())
  }
}
// endregion: --- Tests
