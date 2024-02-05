use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug, PartialEq, Eq)]
struct GameData {
    value: char,
    location: Point,
    good: bool,

}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}


fn main() {
    let input_data = read_input("src/input.txt").unwrap();

    let coordinate_vec = create_mapping(&input_data);

    let special_chars = find_special_characters(input_data);

    let valid_nums = map_values_finder(coordinate_vec, special_chars);

    let good_nums = get_good_nums(valid_nums);

    let total: i32 = good_nums.iter().sum();

    println!("{}", total);
}


fn get_good_nums(coordinate_vec: Vec<GameData>) -> Vec<i32> {
    let mut good_nums: Vec<i32> = Vec::new();

    let mut num_string = String::new();
    let mut good_num: bool = false;

    for coord in coordinate_vec {
        if good_num == false {
            good_num = coord.good;
        }
        if coord.value.is_numeric() {
            num_string = num_string + coord.value.to_string().as_str();
            if good_num == false {
                good_num = coord.good;
            }
        } else {
            if good_num == true {
                if let Ok(parsed_num) = num_string.parse::<i32>() {
                    good_nums.push(parsed_num);
                } else {
                    // Do nothing
                }
                
            }
            num_string = String::new();
            good_num = false;

        }
        
    }

    good_nums
}


fn map_values_finder(mut coordinate_vec: Vec<GameData>, special_chars: Vec<char>) -> Vec<GameData> {
    let mut valid_nums: Vec<GameData> = Vec::new();

    for coord in &coordinate_vec {
        let current_char = coord.value;

        if current_char.is_numeric() {
            let target_x = coord.location.x;
            let target_y = coord.location.y;

            let perimiter_values = perimiter_mapping(target_x, target_y);

            for values in perimiter_values {
                let adjacent_value = find_element(&coordinate_vec, values.x, values.y);
                if special_chars.contains(&adjacent_value.unwrap()) {
                    valid_nums.push(GameData { value: current_char, location: Point { x: target_x, y: target_y }, good: true })
                }
            }
        }
    }

    for updated_coord in valid_nums {
        if let Some(coord) = coordinate_vec.iter_mut().find(|c| c.location == updated_coord.location) {
            *coord = updated_coord;
        }
    }


    coordinate_vec
}


fn perimiter_mapping(x: i32, y: i32) -> Vec<Point> {
    let mut perimiter_vec: Vec<Point> = Vec::new();

    perimiter_vec.push(Point{ x: x-1, y: y-1 });
    perimiter_vec.push(Point{ x: x, y: y-1 });
    perimiter_vec.push(Point{ x: x+1, y: y-1 });
    perimiter_vec.push(Point{ x: x-1, y: y});
    perimiter_vec.push(Point{ x: x+1, y: y});
    perimiter_vec.push(Point{ x: x-1, y: y+1 });
    perimiter_vec.push(Point{ x: x, y: y+1});
    perimiter_vec.push(Point{ x: x+1, y: y+1 });

    perimiter_vec
}


fn find_element(coordinate_vec: &Vec<GameData>, target_x: i32, target_y: i32) -> Option<char> {
    if let Some(coord) = coordinate_vec.iter().find(|&coord| coord.location.x == target_x && coord.location.y == target_y) {
        Some(coord.value)
    } else {
        Some('.') 
    }
}


fn create_mapping(input_data: &Vec<String>) -> Vec<GameData> {
    let mut coordinate_vec: Vec<GameData> = Vec::new();

    let mut y: i32 = 0;
    for row in input_data {
        let mut x: i32 = 0;

        for character in row.chars() {
            let value = character;
            let location = Point { x: x, y: y };
            let game_coord = GameData { value: value, location: location, good: false };
            coordinate_vec.push(game_coord);
            x += 1;
        }
        y += 1;
    }

    coordinate_vec
}


fn find_special_characters(input_data: Vec<String>) -> Vec<char> {
    let mut special_chars: Vec<char> = Vec::new();

    for row in input_data {
        for character in row.chars(){
            
            if !character.is_numeric() && character != '.' {
                special_chars.push(character)
            }
        }
    }

    special_chars
}


fn read_input(file_path: &str) -> std::io::Result<Vec<String>> {
    let f = File::open(file_path)?;
    let mut reader: BufReader<File> = BufReader::new(f);
    let mut contents: String = String::new();
    reader.read_to_string(&mut contents)?;
    let contents_vec: Vec<String> = contents.lines().map(String::from).collect();
    Ok(contents_vec)
}