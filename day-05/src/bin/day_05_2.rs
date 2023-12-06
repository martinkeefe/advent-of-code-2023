fn main() {
    println!("{}", process(include_str!("./input.txt")));
}

// -----------------------------------------------------------------------------
// Types

#[derive(Debug, Clone)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<Map>,
}

#[derive(Debug, Clone, Copy)]
struct MapRange {
    src_start: u64,
    dst_start: u64,
    len: u64,
}

type Map = Vec<MapRange>;

// -----------------------------------------------------------------------------
// Parsing
//
// seeds: 79 14 55 13
//
// seed-to-soil map:
// 50 98 2
// 52 50 48
//
// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15
//
// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4
//
// water-to-light map:
// 88 18 7
// 18 25 70
//
// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13
//
// temperature-to-humidity map:
// 0 69 1
// 1 0 69
//
// humidity-to-location map:
// 60 56 37
// 56 93 4

fn parse_seeds(line: &str) -> Vec<u64> {
    let raw = line
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();

    raw.chunks(2)
        .collect::<Vec<&[u64]>>()
        .iter()
        .map(|&n| ((n[0])..(n[0] + n[1])).collect())
        .collect::<Vec<Vec<u64>>>()
        .concat()
}

fn parse_range(line: &&str) -> MapRange {
    let ns: Vec<u64> = line.split(' ').map(|s| s.parse().unwrap()).collect();

    MapRange {
        src_start: ns[1],
        dst_start: ns[0],
        len: ns[2],
    }
}

fn parse_almanac(input: &str) -> Almanac {
    let lines: Vec<&str> = input.lines().collect();

    Almanac {
        seeds: parse_seeds(lines[0]),
        maps: lines[2..]
            .split(|line| line.is_empty())
            .map(|s| s[1..].iter().map(parse_range).collect())
            .collect::<Vec<Map>>(),
    }
}

// -----------------------------------------------------------------------------
// Solution

fn map_item(map: &Map, item: u64) -> u64 {
    for range in map {
        if item >= range.src_start && item < range.src_start + range.len {
            let i = item - range.src_start;
            return range.dst_start + i;
        }
    }
    item
}

fn map_seed(maps: &Vec<Map>, seed: u64) -> u64 {
    let mut item = seed;
    for map in maps {
        item = map_item(map, item)
    }
    item
}

fn process(input: &str) -> u64 {
    let almanac = parse_almanac(input);

    almanac
        .seeds
        .iter()
        .map(|seed| map_seed(&almanac.maps, *seed))
        .min()
        .unwrap()
}

// -----------------------------------------------------------------------------
// Testing

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sample() {
        assert_eq!(process(include_str!("./sample.txt")), 46)
    }
}
