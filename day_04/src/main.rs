use std::fs::File;
use std::io::{BufReader, Read};



fn main() {
    let file = read_input("src/input.txt").unwrap();

    let mut remove_card_info: Vec<String> = Vec::new();

    for f in file {
        let x = split_string(f.as_str(), ":");
        remove_card_info.push(x[1].to_string())
    }

    let mut totals: usize = 0;


    for r in remove_card_info {
        let numbers_check = split_string(r.as_str(), "|");

        let left_numbers = extract_numbers(numbers_check[0]);
        let right_numbers = extract_numbers(numbers_check[1]);

        let intersection = find_intersection(left_numbers, right_numbers);

        let total_matches = intersection.len();

        if total_matches > 0 {
            let mut inner_totals: usize = 1;
            for _ in 0..total_matches - 1 {
                inner_totals *= 2;
                println!("{}", inner_totals);
            }

            totals += inner_totals;
        }
    }

    println!("{}", totals)
}


fn find_intersection(vec1: Vec<i32>, vec2: Vec<i32>) -> Vec<i32> {
    let mut intersection = Vec::new();

    for item in vec1 {
        if vec2.contains(&item) && !intersection.contains(&item) {
            intersection.push(item)
        }
    }

    intersection
}


fn extract_numbers(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect()
}


fn split_string<'a>(string: &'a str, splitter: &str) -> Vec<&'a str> {
    let parts: Vec<&str> = string.split(splitter).collect();

    parts
}

fn read_input(file_path: &str) -> std::io::Result<Vec<String>> {
    let f = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(f);
    let mut contents: String = String::new();
    reader.read_to_string(&mut contents)?;
    let contents_vec: Vec<String> = contents.lines().map(String::from).collect();
    Ok(contents_vec)
}