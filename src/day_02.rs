use crate::Puzzle;

fn get_cubes(input: &str) -> (i32, Vec<Vec<(i32, &str)>>) {
  let game = input.split(": ").collect::<Vec<_>>();
  let game_id = game[0].replace("Game ", "").parse::<i32>().unwrap();

  let sets_cubes = game[1].split("; ");

  let cubes = sets_cubes.map(|v| v.split(", ").collect::<Vec<_>>());

  let cubes = cubes
    .map(|cube| {
      let color_and_amount = cube.iter().copied().map(|item| {
        let vec = item.split(' ').collect::<Vec<_>>();
        let amount = vec[0].parse::<i32>().unwrap();
        let color = vec[1];
        (amount, color)
      });

      color_and_amount.collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  (game_id, cubes)
}

fn get_valid_game_id(input: &str) -> i32 {
  let (game_id, cubes) = get_cubes(input);

  for cube in cubes {
    for (amount, color) in cube {
      if (amount > 12 && color == "red")
        || (amount > 13 && color == "green")
        || (amount > 14 && color == "blue")
      {
        return 0; // does not satisfy the game
      }
    }
  }

  game_id // game ok
}

fn get_power_of_sets(input: &str) -> i32 {
  let (_, cubes) = get_cubes(input);

  let mut reds: Vec<i32> = vec![];
  let mut blues: Vec<i32> = vec![];
  let mut greens: Vec<i32> = vec![];

  for cube in cubes {
    for (amount, color) in cube {
      match color {
        "red" => reds.push(amount),
        "green" => greens.push(amount),
        "blue" => blues.push(amount),
        _ => (),
      }
    }
  }

  let max_red = reds.iter().max().unwrap();
  let max_blue = blues.iter().max().unwrap();
  let max_green = greens.iter().max().unwrap();

  max_red * max_blue * max_green
}

fn get_sum_ids_of_game(input: &str) -> i32 {
  let games = input.trim().split('\n');
  let valid_ids: Vec<i32> = games.map(|g| get_valid_game_id(g.trim())).collect();
  valid_ids.iter().sum()
}

fn get_sum_of_power_of_these_sets(input: &str) -> i32 {
  let games = input.trim().split('\n');
  let power_of_sets: Vec<i32> = games.map(|g| get_power_of_sets(g.trim())).collect();
  power_of_sets.iter().sum()
}

pub fn play(input: &str, puzzle: &Puzzle) -> i32 {
  if puzzle.eq(&Puzzle::Part01) {
    get_sum_ids_of_game(input)
  } else {
    get_sum_of_power_of_these_sets(input)
  }
}

// region:    --- Tests
#[cfg(test)]
mod tests {
  use super::*;
  use anyhow::Result;

  #[test]
  fn test_sum_ids_of_game_part01_ok() -> Result<()> {
    let fx_input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#;
    let fx_expect = 8;

    let rs = play(fx_input, &Puzzle::Part01);

    assert_eq!(rs, fx_expect);

    Ok(())
  }

  #[test]
  fn test_get_sum_of_power_part02_ok() -> Result<()> {
    let fx_input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#;
    let fx_expect = 2286;

    let rs = play(fx_input, &Puzzle::Part02);

    assert_eq!(rs, fx_expect);

    Ok(())
  }
}
// endregion: --- Tests
