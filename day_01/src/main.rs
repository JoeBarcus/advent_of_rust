use std::fs::File;
use std::io::{BufReader, Read};
use std::collections::HashMap;


fn read_input(file_path: &str) -> std::io::Result<Vec<String>> {
    let f = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(f);
    let mut contents: String = String::new();
    reader.read_to_string(&mut contents)?;
    let contents_vec: Vec<String> = contents.lines().map(String::from).collect();
    Ok(contents_vec)
}


fn find_numbers(input_row: &String) -> u32 {
    let mut left: char = '0';
    let mut right: char = '0';

    for character in input_row.chars() {
        if character.is_numeric() {
            left = character;
            break;
        }
    }

    for character in input_row.chars().rev() {
        if character.is_numeric() {
            right = character;
            break;
        }
    }

    let chars_concat = format!("{}{}", left, right);
    let mut sum_numbers: u32 = 0;
    match chars_concat.parse::<u32>() {
        Ok(parsed_number) => {
            sum_numbers = parsed_number;
        }
        Err(_) => {
            println!("Error: Unable to parse the string as an i32");
        }
    }

    sum_numbers
}


fn find_numbers_two(input_row: &String) -> u32 {
    let number_map: HashMap<&str, &str> = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]
    .iter()
    .cloned()
    .collect();

    let mut location_map: HashMap<String, usize> = HashMap::new();

    for (&key, &value) in number_map.iter() {
        
        if input_row.contains(key) {
            let location = input_row.find(key).unwrap();
            location_map.insert(key.to_string(), location);
        }

        if input_row.contains(value) {
            let location = input_row.find(value).unwrap();
            location_map.insert(value.to_string(), location);
        }
    }

    let mut min_key = String::new();
    let mut max_key = String::new();

    if let Some((key, _min_value)) = location_map.iter().min_by_key(|&(_, value)| value) {
        min_key = key.to_string();
    }

    if let Some((key, _max_value)) = location_map.iter().max_by_key(|&(_, value)| value) {
        max_key = key.to_string();
    }

    let left = match number_map.get(&min_key.as_str()) {
                            Some(&value) => &value,
                            None => min_key.as_str(),
                        };

    let right = match number_map.get(&max_key.as_str()) {
                            Some(&value) => &value,
                            None => max_key.as_str(),
                        };

    let nums_as_string = left.to_string() + right;

    let total = match nums_as_string.parse::<u32>() {
        Ok(value) => value,
        Err(_) => 99999,
    };

    //println!{"{:?}", location_map}
    //println!("{}", total);
    total
}

fn main() {
    // Read in file
    let mut input_data: Vec<String> = Vec::new();
    match read_input("src/input.txt") {
        Ok(contents) => {
            input_data = contents;
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }


    // Part One
    let mut part_one: u32 = 0;
    for input_row in &input_data {
        let row_sum: u32 = find_numbers(input_row);
        part_one += row_sum;
    }

    println!("Part 1 answer: {}", part_one);


    // Part Two
    let mut part_two: u32 = 0;
    for input_row in &input_data {
        let row_sum: u32 = find_numbers_two(input_row);
        part_two += row_sum;
    }

    println!("Part 2 answer: {}", part_two);

}
