use std::collections::HashMap;

use crate::tools::vec_lines;
use regex::Regex;

struct World {
    map: String,
    width: u32,
    height: u32,
}

impl World {
    fn adjacents_chars(&self, index: u32) -> Vec<Adjacency> {
        adjacents(index, self.width, self.height)
            .iter()
            .map(|idx| (*idx, self.map.chars().nth(*idx as usize).unwrap()))
            .collect()
    }

    fn from_lines(lines: Vec<String>) -> World {
        let height = lines.len() as u32;
        let map = lines.concat();
        let width = map.len() as u32 / height;
        World { map, width, height }
    }

    fn get_part_numbers(&self) -> (Vec<u32>, HashMap<u32, Vec<u32>>) {
        let symbols = Regex::new(r"[^\.0-9]").unwrap();
        let iter = self.map.chars();
        let mut buffer = "".to_string();
        let mut adjacency_buffer = vec![];
        let mut result = vec![];
        let mut gears = HashMap::<u32, Vec<u32>>::new();
        iter.enumerate().for_each(|(idx, c)| {
            if c.is_digit(10) {
                buffer.push(c);
                adjacency_buffer.extend(self.adjacents_chars(idx as u32));
            }
            if (!c.is_digit(10) || idx as u32 % self.width == self.width - 1) && buffer.len() > 0 {
                adjacency_buffer.dedup_by(|a, b| a.0 == b.0);

                // println!("buf {:?}", buffer);
                // println!("adj {:?}", adjacency_buffer);

                if symbols.is_match(
                    adjacency_buffer
                        .iter()
                        .map(|(_, c)| c)
                        .collect::<String>()
                        .as_str(),
                ) {
                    let part_number = buffer.parse().unwrap();
                    result.push(part_number);
                    adjacency_buffer.iter().for_each(|(idx, c)| {
                        if *c == '*' {
                            if !gears.contains_key(idx) {
                                gears.insert(*idx, vec![]);
                            }
                            gears.get_mut(idx).unwrap().push(part_number);
                        }
                    })
                }
                adjacency_buffer.clear();
                buffer.clear();
            }
        });
        gears.retain(|_, nbs| {
            nbs.dedup();
            return nbs.len() == 2;
        });
        (result, gears)
    }
}

type Adjacency = (u32, char);

fn adjacents(index: u32, width: u32, height: u32) -> Vec<u32> {
    let iwidth = width as i32;
    let iheight = height as i32;
    let iindex = index as i32;

    let mut adjacency = vec![];

    match iindex % iwidth {
        0 => {
            // first column
            adjacency.extend_from_slice(&vec![
                iindex - iwidth,
                iindex - iwidth + 1,
                iindex + 1,
                iindex + iwidth,
                iindex + iwidth + 1,
            ])
        }
        i if i == iwidth - 1 => {
            // last column
            adjacency.extend_from_slice(&vec![
                iindex - iwidth,
                iindex - iwidth - 1,
                iindex - 1,
                iindex + iwidth,
                iindex + iwidth - 1,
            ])
        }
        _ => {
            // last column
            adjacency.extend_from_slice(&vec![
                iindex - iwidth - 1,
                iindex - iwidth,
                iindex - iwidth + 1,
                iindex - 1,
                iindex + 1,
                iindex + iwidth - 1,
                iindex + iwidth,
                iindex + iwidth + 1,
            ])
        }
    }
    adjacency
        .iter()
        .filter_map(|f| {
            if *f >= 0 && *f < iwidth * iheight {
                Some(*f as u32)
            } else {
                None
            }
        })
        .collect()
}

pub fn run() {
    let (parts_numbers, gears) = World::from_lines(vec_lines("3_input.txt")).get_part_numbers();
    println!("Parts number: {:?}", parts_numbers.iter().sum::<u32>());
    println!(
        "Gear ratio sum: {:?}",
        gears
            .iter()
            .map(|(_, gear)| return gear.get(0).unwrap() * gear.get(1).unwrap())
            .sum::<u32>()
    )
}

#[cfg(test)]
mod tests {
    use crate::e3::adjacents;

    use super::World;

    #[test]
    fn adjacent_simple() {
        // 01234
        // 56789
        // 01234
        // 56789
        assert_eq!(adjacents(0, 5, 4), vec![1, 5, 6]);
        assert_eq!(adjacents(5, 5, 4), vec![0, 1, 6, 10, 11]);
        assert_eq!(adjacents(2, 5, 4), vec![1, 3, 6, 7, 8]);
        assert_eq!(adjacents(7, 5, 4), vec![1, 2, 3, 6, 8, 11, 12, 13]);
        assert_eq!(adjacents(19, 5, 4), vec![14, 13, 18]);
    }

    #[test]
    fn adjacents_chars_simple() {
        let w = World {
            map: "01234567890123456789".to_string(),
            width: 5,
            height: 4,
        };
        assert_eq!(w.adjacents_chars(0), vec![(1, '1'), (5, '5'), (6, '6')]);
        assert_eq!(
            w.adjacents_chars(14),
            vec![(9, '9'), (8, '8'), (13, '3'), (19, '9'), (18, '8')]
        );
    }

    #[test]
    fn build_world() {
        let w = World::from_lines(vec![
            "01234".to_string(),
            "56789".to_string(),
            "01234".to_string(),
            "56789".to_string(),
        ]);
        assert_eq!(w.map, "01234567890123456789".to_string());
        assert_eq!(w.height, 4);
        assert_eq!(w.width, 5);
    }

    #[test]
    fn parse_numbers() {
        let w0 = World {
            map: ".212.5..9812345.....".to_string(),
            width: 5,
            height: 4,
        };
        assert_eq!(w0.get_part_numbers().0, vec![]);
        let w1 = World {
            map: "*212.5//9812345*....".to_string(),
            width: 5,
            height: 4,
        };
        assert_eq!(w1.get_part_numbers().0, vec![212, 5, 98, 12345]);
        let w2 = World {
            map: "*212.5..9812345.....".to_string(),
            width: 5,
            height: 4,
        };
        assert_eq!(w2.get_part_numbers().0, vec![212, 5]);
        let w3 = World {
            map: "467..114.....*........35..633.......#...617*...........+.58...592...........755....$.*.....664.598..".to_string(),
            height: 10,
            width: 10,
        };
        assert_eq!(
            w3.get_part_numbers().0,
            vec![467, 35, 633, 617, 592, 755, 664, 598]
        );
        assert_eq!(w3.get_part_numbers().1.get(&13).unwrap(), &vec![467, 35]);
        assert_eq!(w3.get_part_numbers().1.get(&85).unwrap(), &vec![755, 598]);
        assert!(w3.get_part_numbers().1.get(&43).is_none());
    }
}
