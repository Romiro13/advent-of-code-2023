pub enum Puzzle {
  Part01,
  Part02,
}

fn get_valid_ids(input: &str) -> i32 {
  let v = input.split(": ").collect::<Vec<&str>>();
  let id = v[0].replace("Game ", "");

  let cubes = v[1].split("; ");

  let cubes = cubes.map(|v| v.split(", ").collect::<Vec<_>>());

  let cubes = cubes
    .map(|color_count| {
      let color_and_count = color_count.iter().copied();
      let color_and_count = color_and_count.map(|c| {
        let v = c.split(' ').collect::<Vec<_>>();
        (v[0].parse::<i32>().unwrap(), v[1])
      });

      color_and_count.collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  for c in cubes {
    for m in c {
      if (m.0 > 12 && m.1 == "red") || (m.0 > 13 && m.1 == "green") || (m.0 > 14 && m.1 == "blue") {
        return 0;
      }
    }
  }

  id.parse::<i32>().unwrap()
}

pub fn sum_ids_of_game(input: &str) -> i32 {
  let games = input.trim().split('\n');
  let valid_ids: Vec<i32> = games.map(|g| get_valid_ids(g.trim())).collect();
  valid_ids.iter().sum()
}

// region:    --- Tests
#[cfg(test)]
mod tests {
  use super::*;
  use anyhow::Result;

  #[test]
  fn test_sum_ids_of_game_ok() -> Result<()> {
    let fx_input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#;
    let fx_expect = 8;

    let rs = sum_ids_of_game(fx_input);

    assert_eq!(rs, fx_expect);

    Ok(())
  }
}
// endregion: --- Tests
