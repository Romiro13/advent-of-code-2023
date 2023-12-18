use crate::Puzzle;

fn find_numbers(input: &str, puzzle: &Puzzle) -> String {
  let text_digits = vec![
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
  ];

  let mut numbers: Vec<(usize, &str)> = Vec::new();

  for (text, digit) in text_digits {
    let mut digits_numbers: Vec<(usize, &str)> = input.match_indices(digit).collect();

    if puzzle.eq(&Puzzle::Part02) {
      let numbers_words: Vec<(usize, &str)> = input.match_indices(text).collect();
      if !numbers_words.is_empty() {
        let mut word_to_digit = numbers_words.iter().map(|v| (v.0, digit)).collect();
        digits_numbers.append(&mut word_to_digit);
      }
    }

    numbers.append(&mut digits_numbers);
  }

  numbers.sort_by_key(|k| k.0);

  let numbers: Vec<&str> = numbers.iter().map(|v| v.1).collect();

  numbers.concat()
}

fn get_first_and_last_number(input: &str, puzzle: &Puzzle) -> i32 {
  let numbers = find_numbers(input, puzzle);

  let number = format!(
    "{}{}",
    numbers
      .chars()
      .take(1)
      .last()
      .expect("Couldn't get the first char"),
    numbers.chars().last().expect("Couldn't get the last char")
  );

  number
    .parse::<i32>()
    .unwrap_or_else(|_| panic!("Unable to convert '{}' to 'i32'", number))
}

pub fn get_sum_calibration_value(input: &str, puzzle: &Puzzle) -> i32 {
  let lines = input.split('\n');
  let numbers: Vec<i32> = lines
    .map(move |line| get_first_and_last_number(line.trim(), puzzle))
    .collect();
  numbers.iter().sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_calibration_value_ok() {
    //Setup
    let fx_input = r#"1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet"#;
    let fx_expect = 142;

    //Exec
    let rs = get_sum_calibration_value(fx_input, &Puzzle::Part01);

    //Check
    assert_eq!(fx_expect, rs);
  }

  #[test]
  fn test_calibration_value_with_text_numbers_ok() {
    //Setup
    let fx_input = r#"two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen"#;
    let fx_expect = 281;

    //Exec
    let rs = get_sum_calibration_value(fx_input, &Puzzle::Part02);

    //Check
    assert_eq!(fx_expect, rs);
  }
}
