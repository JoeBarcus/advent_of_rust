use std::array;
use std::fs::File;
use std::io::{BufReader, Read};
use std::collections::HashMap;


#[derive(Debug, Clone)]
struct Range {
    start: u64,
    length: u64
}


#[derive(Debug, Clone)]
struct Map {
    source_range: Range,
    dest_range: Range,
}


fn main() {
    let mut input_data: Vec<String> = Vec::new();
    match read_input("src/input.txt") {
        Ok(contents) => input_data = contents,
        Err(err) => eprintln!("Error reading file: {}", err),
    }

    let mapping_categories: [&str; 7] = [
        "seed-to-soil map:",
        "soil-to-fertilizer map:",
        "fertilizer-to-water map:",
        "water-to-light map:",
        "light-to-temperature map:",
        "temperature-to-humidity map:",
        "humidity-to-location map:",
    ];

    let seed_nums: Vec<u64> = input_data[0].split_whitespace().filter_map(|x| x.parse().ok()).collect();
    let map_hash = create_mappings(mapping_categories, input_data.clone());

    //println!("{:?}", map_hash);

    let part_one = solve_part_one(mapping_categories, map_hash, seed_nums);

    println!("{}", part_one);
   
}


fn solve_part_one(mapping_categories: [&str; 7], map_hash: HashMap<String, Vec<Map>>, seed_nums: Vec<u64>) -> u64 {
    let mut final_mapped_values: Vec<u64> = Vec::new();
    for seed in seed_nums {
        let mut mapped_value: u64 = seed;
        let mut chain_mapping: &Vec<Map>;
        for mapping in mapping_categories {
            
            chain_mapping = map_hash.get(mapping).unwrap();

            for maps in chain_mapping {
                
                let range_map = maps;
                let vec_source: Vec<u64> = (range_map.source_range.start..range_map.source_range.start + range_map.source_range.length as u64).collect();
                let vec_dest: Vec<u64> = (range_map.dest_range.start..range_map.dest_range.start + range_map.dest_range.length as u64).collect();
                if let Some(index) = vec_source.iter().position(|&x| x == mapped_value) {
                    mapped_value = vec_dest[index];
                    break;
                } else {
                    mapped_value = mapped_value;
                    continue;
                }
            }
        }
        final_mapped_values.push(mapped_value)
    }

    println!("{:?}", final_mapped_values.clone());
    *final_mapped_values.iter().min().unwrap()
}


fn create_mappings(mappings_categories: [&str; 7], input_data: Vec<String>) -> HashMap<String, Vec<Map>> {
    let mut mapping_hash: HashMap<String, Vec<Map>> = HashMap::new();
    let mut mapping_state = String::new();
    let mut mapping_vec = Vec::new();

    for input in &input_data {
        if input.contains("seeds") {
            continue
        }
        if input == "" || input == "EOF" {
            mapping_hash.insert(mapping_state.clone(), mapping_vec.clone());
            mapping_state = String::new();
            mapping_vec = Vec::new();
            continue
        }
        if mappings_categories.contains(&input.as_str()) {
            mapping_state = input.clone();
            continue
        }
        let input_parsed: Vec<u64> = input.split_whitespace().filter_map(|x| x.parse().ok()).collect();
        let source_range = Range { start: input_parsed[1], length: input_parsed[2]};
        let dest_range = Range { start: input_parsed[0], length: input_parsed[2]};
        let map_range = Map { source_range: source_range, dest_range: dest_range};
        mapping_vec.push(map_range);
        }

    mapping_hash
}


fn read_input(file_path: &str) -> std::io::Result<Vec<String>> {
    let f = File::open(file_path)?;
    let mut reader = BufReader::new(f);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let contents_vec: Vec<String> = contents.lines().map(String::from).collect();
    Ok(contents_vec)
}