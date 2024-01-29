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

fn main() {
    let mut input_data: Vec<String> = Vec::new();
    match read_input("src/input.txt") {
        Ok(contents) => input_data = contents,
        Err(err) => eprintln!("Error reading file: {}", err),
    }

    let mut game_map: HashMap<&str, u32> = HashMap::new();
    game_map.insert("red", 12);
    game_map.insert("green", 13);
    game_map.insert("blue", 14);

    let mut score_totals: u32 = 0;
    for data in input_data {
        let parts: Vec<&str> = data.split(":").collect();
        let mut game_number: u32 = 0;

        for part in parts {
            if part.contains("Game") {
                let game_finder: Vec<&str> = part.split(" ").collect();
                match game_finder[1].parse::<u32>() {
                    Ok(parsed_number) => {
                        game_number = parsed_number;
                    }
                    Err(e) => {
                        println!("Failed to parse: {}", e);
                    }
                }
            } else {
                let game_scores: Vec<&str> = part.split(";").collect();
                for game in game_scores {
                    let get_score: Vec<&str> = game.split(",").collect();
                    for score in get_score {
                        let score = score.trim();
                        let extract_score: Vec<&str> = score.split(" ").collect();
                        let actual_score = extract_score[0];
                        let mut the_score: u32 = 0;
                        match actual_score.parse::<u32>() {
                            Ok(parsed_number) => {
                                the_score = parsed_number;
                            }
                            Err(e) => {
                                println!("Failed to parse: {}", e);
                            }
                        }                     
                        match game_map.get(extract_score[1]) {
                            Some(value) => {
                                if value < &the_score {
                                    game_number = 0;
                                }
                            }
                            None => println!("Can't find {}", extract_score[1])
                        }
                    }
                }
            }
        }
        score_totals = score_totals + game_number;
    }
    println!("Total = {}", score_totals);
}