#![allow(dead_code)]

use std::{fs, io::Read};

#[tokio::main]
async fn main() {
    const INPUT_FILE: &str = "input.txt";

    let mut buffer = String::new();
    fs::File::open(INPUT_FILE)
        .unwrap()
        .read_to_string(&mut buffer)
        .unwrap();

    let input = Input::from_str(&buffer);
    println!("input: {:#?}", input);

    println!("p1_min_location: {}", p1_min_location(&input));

    println!("p2_min_location: {}", p2_min_location(input).await);
}

async fn p2_min_location(input: Input) -> isize {
    let mut idx = 0;
    let mut async_input: Vec<(isize, isize)> = Vec::new();
    while idx < input.seeds.len() {
        let (start, range) = (input.seeds[idx], input.seeds[idx + 1]);
        println!("{}, {}", start, range);
        async_input.push((start, range));

        // for i in 0..range {
        //     let seed = start + i;

        //     let location = p1_get_seed_location(&seed, input);
        //     min_location = min_location.min(location);
        // }

        idx += 2;
    }

    let mut handles = Vec::new();
    let mut results = Vec::new();
    for (start, range) in async_input {
        let cloned = input.clone();
        handles.push(tokio::task::spawn(async move {
            p2_min_location_async(start, range, cloned.clone())
        }));
    }

    for h in handles {
        results.push(h.await.unwrap());
    }

    let mut min_location = isize::MAX;
    for r in results {
        min_location = min_location.min(r);
    }

    min_location
}

fn p2_min_location_async(start: isize, range: isize, input: Input) -> isize {
    println!("start {} {}", start, range);

    let mut min_location = isize::MAX;

    for i in 0..range {
        let seed = start + i;

        if seed % 1000000 == 0 {
            println!("working {} {}: {}", start, range, seed)
        }

        let location = p1_get_seed_location(&seed, &input);
        min_location = min_location.min(location)
    }

    println!("done {} {}", start, range);

    min_location
}

fn p1_min_location(input: &Input) -> isize {
    let mut min_location: isize = isize::MAX;

    for s in input.seeds.iter() {
        let location = p1_get_seed_location(s, input);
        // println!("seed {} -> location {}", s, location);
        min_location = min_location.min(location);
    }

    min_location
}

fn p1_get_seed_location(seed: &isize, input: &Input) -> isize {
    // println!("\nseed: {}", seed);
    let soil = p1_get_dest_from_ranges(&input.seed_to_soil, *seed);
    // println!("soil: {}", soil);
    let fertilizer = p1_get_dest_from_ranges(&input.soil_to_fertilizer, soil);
    // println!("fertilizer: {}", fertilizer);
    let water = p1_get_dest_from_ranges(&input.fertilizer_to_water, fertilizer);
    // println!("water: {}", water);
    let light = p1_get_dest_from_ranges(&input.water_to_light, water);
    // println!("light: {}", light);
    let temperature = p1_get_dest_from_ranges(&input.light_to_temperature, light);
    // println!("temperature: {}", temperature);
    let humidity = p1_get_dest_from_ranges(&input.temperature_to_humidity, temperature);
    // println!("humidity: {}", humidity);
    let location = p1_get_dest_from_ranges(&input.humidity_to_location, humidity);
    // println!("location: {}", location);

    location
}

fn p1_get_dest_from_ranges(ranges: &Vec<Range>, src: isize) -> isize {
    // binary search
    let (mut l, mut r) = (0, ranges.len() - 1);

    while l <= r {
        let mid = l + (r - l) / 2;
        // is mid
        let range = &ranges[mid];
        if src >= range.src_start && src <= range.src_end {
            return range.dst_start + (src - range.src_start);
        }
        if src < range.src_start {
            if mid == 0 {
                break;
            }
            r = mid - 1;
        } else {
            l = mid + 1;
        }
    }

    // for r in ranges {
    //     if src >= r.src_start && src <= r.src_end {
    //         return r.dst_start + (src - r.src_start);
    //     }
    //     if src < r.src_start {
    //         break;
    //     }
    // }

    src
}

#[derive(Default, Debug, Clone)]
struct Input {
    seeds: Vec<isize>,
    seed_to_soil: Vec<Range>,
    soil_to_fertilizer: Vec<Range>,
    fertilizer_to_water: Vec<Range>,
    water_to_light: Vec<Range>,
    light_to_temperature: Vec<Range>,
    temperature_to_humidity: Vec<Range>,
    humidity_to_location: Vec<Range>,
}

#[derive(Default, Debug, Clone, Copy)]
struct Range {
    src_start: isize,
    src_end: isize,
    dst_start: isize,
}

impl Input {
    fn from_str(data: &str) -> Self {
        let mut input = Input::default();

        let mut lines: Vec<&str> = data.lines().collect(); // add a last empty line to parse the last map
        lines.push("");

        // parse seeds
        let seeds: Vec<isize> = lines[0]
            .replace("seeds:", "")
            .trim()
            .split(' ')
            .map(|v| v.parse::<isize>().unwrap())
            .collect();
        input.seeds = seeds;

        let mut ranges_name = "";
        let mut ranges: Vec<Range> = Vec::new();
        for l in lines.iter().skip(2) {
            // set map
            if l.is_empty() {
                ranges.sort_by(|a, b| a.src_start.cmp(&b.src_start));

                match ranges_name {
                    "seed-to-soil map:" => {
                        input.seed_to_soil = ranges.clone();
                    }
                    "soil-to-fertilizer map:" => {
                        input.soil_to_fertilizer = ranges.clone();
                    }
                    "fertilizer-to-water map:" => {
                        input.fertilizer_to_water = ranges.clone();
                    }
                    "water-to-light map:" => {
                        input.water_to_light = ranges.clone();
                    }
                    "light-to-temperature map:" => {
                        input.light_to_temperature = ranges.clone();
                    }
                    "temperature-to-humidity map:" => {
                        input.temperature_to_humidity = ranges.clone();
                    }
                    "humidity-to-location map:" => {
                        input.humidity_to_location = ranges.clone();
                    }
                    _ => {
                        panic!("unknown map name: {}", ranges_name);
                    }
                }

                println!("map {} updated", ranges_name);

                ranges = Vec::new();
                ranges_name = "";
                continue;
            }
            // set map name
            if ranges_name.is_empty() {
                ranges_name = l;
                continue;
            }
            // parse map data
            let mapping: Vec<isize> = l.split(' ').map(|v| v.parse::<isize>().unwrap()).collect();
            if mapping.len() != 3 {
                panic!("invalid mapping {:#?}", mapping);
            }
            let (dest, source, range) = (mapping[0], mapping[1], mapping[2]);
            ranges.push(Range {
                src_start: source,
                src_end: source + range - 1,
                dst_start: dest,
            });
        }

        input
    }
}
