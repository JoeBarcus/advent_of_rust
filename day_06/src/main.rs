use std::fs::File;
use std::io::{BufReader, Read};


#[derive(Debug)]
struct RaceData {
    time: u32,
    distance: u32,
}


impl RaceData {
    fn time_to_travel(&self, speed: u32) -> u32 {
        (self.time - speed) * speed
    }
}



fn main() {
    let race_data = read_input("src/inputs.txt").unwrap();

    let race_vec_data = parse_race_data(race_data);
    
    let race_winners = calculate_winners(race_vec_data);
    println!("{}", race_winners);

}


fn calculate_winners(race_vec_data: Vec<RaceData>) -> u32 {
    let mut winners: Vec<u32> = Vec::new();
    for race in race_vec_data {
        let mut winner_count: u32 = 0;
        for i in 1..=race.time {
            let distance = race.time_to_travel(i);
            if distance > race.distance {
                winner_count += 1;
            }
        }
        winners.push(winner_count);

    }

    winners.iter().fold(1, |acc, &x| acc * x)
}


fn parse_race_data(race_data: Vec<String>) -> Vec<RaceData> {
    let time_data: Vec<u32> = race_data[0].split_whitespace().filter_map(|x| x.parse().ok()).collect();
    let distance_data: Vec<u32> = race_data[1].split_whitespace().filter_map(|x| x.parse().ok()).collect();
    let mut race_vec_data = Vec::new();

    for (index, value) in time_data.iter().enumerate() {
        race_vec_data.push(RaceData { time: time_data[index], distance: distance_data[index]});
    }

    race_vec_data
}


fn read_input(file_path: &str) -> std::io::Result<Vec<String>> {
    let f = File::open(file_path)?;
    let mut reader = BufReader::new(f);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    let contents_vec: Vec<String> = contents.lines().map(String::from).collect();
    Ok(contents_vec)
}