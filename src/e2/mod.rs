use crate::tools::vec_lines;
use regex::Regex;

#[derive(PartialEq, Eq, Debug)]
struct RGB {
    r: u32,
    g: u32,
    b: u32,
}

impl RGB {
    fn possible_in(&self, other: &Self) -> bool {
        if self.r <= other.r && self.g <= other.g && self.b <= other.b {
            return true;
        }
        return false;
    }

    fn power(&self) -> u32 {
        self.r * self.g * self.b
    }
}

type Game = (u32, Vec<RGB>);

fn game_from_line(source: String) -> Game {
    let (game_str, cubes_str) = source.split_once(":").unwrap();

    let game_regex = Regex::new(r"Game ([0-9]+)").unwrap();
    let (_, [game_number]): (&str, [&str; 1]) = game_regex.captures(game_str).unwrap().extract();

    let color_regex = Regex::new(r"(?<n>[0-9]*) (?<c>blue|green|red)").unwrap();

    let mut cube_vec = vec![];

    cubes_str
        .split(";")
        .map(|pulled| pulled.trim())
        .for_each(|pulled| {
            let mut new_cube = RGB { r: 0, g: 0, b: 0 };
            pulled
                .split(",")
                .map(|color| color.trim())
                .for_each(|color| {
                    let cap = color_regex.captures(color).unwrap();
                    let count = cap.name("n").unwrap().as_str().parse::<u32>().unwrap();
                    match cap.name("c").unwrap().as_str() {
                        "blue" => new_cube.b = count,
                        "green" => new_cube.g = count,
                        "red" => new_cube.r = count,
                        _ => {}
                    };
                });
            cube_vec.push(new_cube);
        });
    return (game_number.parse::<u32>().unwrap(), cube_vec);
}

fn minimum_rgb((_, cubes): Game) -> RGB {
    let mut rgb = RGB { r: 0, g: 0, b: 0 };
    cubes.iter().for_each(|cub| {
        rgb.r = u32::max(rgb.r, cub.r);
        rgb.g = u32::max(rgb.g, cub.g);
        rgb.b = u32::max(rgb.b, cub.b);
    });
    return rgb;
}

fn run_on_text(source: Vec<String>, condition: RGB) -> u32 {
    source
        .iter()
        .map(|line| game_from_line(line.to_owned()))
        .filter_map(|(numb, cubes)| {
            if cubes.iter().all(|c| c.possible_in(&condition)) {
                return Some(numb);
            }
            None
        })
        .sum()
}

fn run_two_on_text(source: Vec<String>) -> u32 {
    source
        .iter()
        .map(|line| minimum_rgb(game_from_line(line.to_owned())).power())
        .sum()
}

pub fn run() {
    println!(
        "Part 1: {:?}",
        run_on_text(
            vec_lines("2_input.txt"),
            RGB {
                r: 12,
                g: 13,
                b: 14
            }
        )
    );
    println!("Part 2: {:?}", run_two_on_text(vec_lines("2_input.txt")));
}

#[cfg(test)]
mod tests {
    use crate::e2::{game_from_line, minimum_rgb, run_on_text, RGB};

    #[test]
    fn interpret_simple() {
        assert_eq!(
            game_from_line(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                    .to_string()
            ),
            (
                3,
                vec![
                    RGB { r: 20, g: 8, b: 6 },
                    RGB { r: 4, g: 13, b: 5 },
                    RGB { r: 1, g: 5, b: 0 }
                ]
            )
        )
    }

    #[test]
    fn power() {
        assert_eq!(RGB { r: 4, g: 2, b: 6 }.power(), 48);
        assert_eq!(RGB { r: 1, g: 3, b: 4 }.power(), 12);
        assert_eq!(RGB { r: 20, g: 13, b: 6 }.power(), 1560);
        assert_eq!(RGB { r: 14, g: 3, b: 15 }.power(), 630);
        assert_eq!(RGB { r: 6, g: 3, b: 2 }.power(), 36);
    }

    #[test]
    fn min_rgb() {
        assert_eq!(
            minimum_rgb(game_from_line(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()
            )),
            RGB { r: 4, g: 2, b: 6 }
        );
        assert_eq!(
            minimum_rgb(game_from_line(
                "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string()
            )),
            RGB { r: 1, g: 3, b: 4 }
        );
        assert_eq!(
            minimum_rgb(game_from_line(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
                    .to_string()
            )),
            RGB { r: 20, g: 13, b: 6 }
        );
        assert_eq!(
            minimum_rgb(game_from_line(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
                    .to_string()
            )),
            RGB { r: 14, g: 3, b: 15 }
        );
        assert_eq!(
            minimum_rgb(game_from_line(
                "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()
            )),
            RGB { r: 6, g: 3, b: 2 }
        );
    }

    #[test]
    fn run_text() {
        let source = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(
            run_on_text(
                source
                    .split("\n")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
                RGB {
                    r: 12,
                    g: 13,
                    b: 14
                }
            ),
            8
        )
    }
}
