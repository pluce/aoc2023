use regex::Regex;

use crate::tools::vec_lines;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct MappingElement {
    source_start: u64,
    destination_start: u64,
    range_length: u64,
}

impl MappingElement {
    pub fn from_line(source: String) -> MappingElement {
        let mut sp = source.split_whitespace();
        MappingElement {
            destination_start: sp.next().map(|x| x.parse::<u64>()).unwrap().unwrap(),
            source_start: sp.next().map(|x| x.parse::<u64>()).unwrap().unwrap(),
            range_length: sp.next().map(|x| x.parse::<u64>()).unwrap().unwrap(),
        }
    }
}

fn traverse_mappings(start: u64, mappings: &Vec<Mapping>) -> u64 {
    mappings.iter().fold(start, |acc, e| e.please_map(acc))
}

struct Mapping {
    defs: Vec<MappingElement>,
}

impl Mapping {
    pub fn sort(&mut self) {
        self.defs
            .sort_by(|a, b| a.source_start.partial_cmp(&b.source_start).unwrap());
    }
    // assert defs is sorted
    pub fn please_map(&self, idx: u64) -> u64 {
        let mut found = false;
        let mut value = 0;
        let mut iter_defs = self.defs.iter();
        let mut def;
        def = iter_defs.next();
        while def.is_some() && !found {
            let the_def = def.unwrap();
            let idx_diff = idx as i64 - the_def.source_start as i64;
            if idx_diff >= 0 && idx_diff < the_def.range_length as i64 {
                found = true;
                value = the_def.destination_start + idx_diff as u64;
            }
            def = iter_defs.next();
        }
        if found {
            return value;
        }
        idx
    }
}

fn parse(source: Vec<String>) -> ParseResult {
    let regex = Regex::new(r"^[0-9 ]*$").unwrap();
    let mut seeds = vec![];
    let mut mappings = vec![];
    let mut current_mapping = vec![];

    for line in source {
        if line.starts_with("seeds: ") {
            let seed_str = line.split("seeds: ").nth(1).unwrap();
            seeds = seed_str
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
            continue;
        }
        if line.is_empty() {
            continue;
        }
        if regex.is_match(&line) {
            current_mapping.push(MappingElement::from_line(line));
            continue;
        } else {
            if (!current_mapping.is_empty()) {
                mappings.push(Mapping {
                    defs: current_mapping,
                });
                current_mapping = vec![];
            }
        }
    }

    let mut mapping = Mapping {
        defs: current_mapping,
    };

    mapping.sort();

    mappings.push(mapping);
    ParseResult { seeds, mappings }
}
pub struct ParseResult {
    seeds: Vec<u64>,
    mappings: Vec<Mapping>,
}

pub fn run() {
    let ps = parse(vec_lines("5_input.txt"));
    let mut result = u64::MAX;
    let mut iter_seed = ps.seeds.iter();
    while let seed_start_opt = iter_seed.next() {
        if (seed_start_opt.is_none()) {
            break;
        }
        let seed_start = seed_start_opt.unwrap();
        let seed_length = iter_seed.next().unwrap();

        println!(
            "Start seed with {:?} for {:?} iterations",
            seed_start, seed_length
        );
        for seed in *seed_start..(*seed_start + *seed_length) {
            if seed % 1000000 == 0 {
                println!("Seed {:?}", seed)
            };
            result = u64::min(result, traverse_mappings(seed, &ps.mappings))
        }
    }
    println!("Min {:?}", result);
}

#[cfg(test)]
mod tests {
    use crate::e5::{traverse_mappings, Mapping, MappingElement};

    use super::parse;

    #[test]
    fn test_parse_all() {
        let lines = FIXTURE
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let ps = parse(lines);
        assert_eq!(ps.mappings.len(), 7);
        assert_eq!(traverse_mappings(55, &ps.mappings), 86);
        assert_eq!(traverse_mappings(79, &ps.mappings), 82);
    }

    #[test]
    fn test_sort() {
        let mut mapping = Mapping {
            defs: vec![
                MappingElement::from_line("50 10 2".to_string()),
                MappingElement::from_line("70 93 9".to_string()),
                MappingElement::from_line("90 0 4".to_string()),
            ],
        };
        mapping.sort();

        assert_eq!(
            mapping.defs,
            vec![
                MappingElement {
                    destination_start: 90,
                    source_start: 0,
                    range_length: 4
                },
                MappingElement {
                    destination_start: 50,
                    source_start: 10,
                    range_length: 2
                },
                MappingElement {
                    destination_start: 70,
                    source_start: 93,
                    range_length: 9
                }
            ]
        );
    }

    #[test]
    fn test_map() {
        let mut mapping = Mapping {
            defs: vec![
                MappingElement::from_line("50 98 2".to_string()),
                MappingElement::from_line("52 50 48".to_string()),
            ],
        };
        mapping.sort();
        assert_eq!(mapping.please_map(79), 81);
        assert_eq!(mapping.please_map(14), 14);
        assert_eq!(mapping.please_map(55), 57);
        assert_eq!(mapping.please_map(13), 13);
    }

    #[test]
    fn test_traverse() {
        let mut mapping_1 = Mapping {
            defs: vec![
                MappingElement::from_line("50 98 2".to_string()),
                MappingElement::from_line("52 50 48".to_string()),
            ],
        };
        mapping_1.sort();

        let mut mapping_2 = Mapping {
            defs: vec![
                MappingElement::from_line("0 15 37".to_string()),
                MappingElement::from_line("37 52 2".to_string()),
                MappingElement::from_line("39 0 15".to_string()),
            ],
        };
        mapping_2.sort();

        let mut mapping_3 = Mapping {
            defs: vec![
                MappingElement::from_line("49 53 8".to_string()),
                MappingElement::from_line("0 11 42".to_string()),
                MappingElement::from_line("42 0 7".to_string()),
                MappingElement::from_line("57 7 4".to_string()),
            ],
        };
        mapping_3.sort();

        let all_maps = vec![mapping_1, mapping_2, mapping_3];
        assert_eq!(traverse_mappings(79, &all_maps), 81);
        assert_eq!(traverse_mappings(14, &all_maps), 49);
        assert_eq!(traverse_mappings(55, &all_maps), 53);
        assert_eq!(traverse_mappings(13, &all_maps), 41);
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            MappingElement::from_line("50 98 2".to_string()),
            MappingElement {
                destination_start: 50,
                source_start: 98,
                range_length: 2
            }
        );
    }

    static FIXTURE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
}
