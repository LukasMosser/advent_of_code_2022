use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let points = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);

    let clear_map = HashMap::from([
        ("A", "Rock"),
        ("B", "Paper"),
        ("C", "Scissors"),
    ]);

    let enc_map = HashMap::from([
        ("X", "A"),
        ("Y", "B"),
        ("Z", "C"),
    ]);   
    
    let wins_map = HashMap::from([
        ("AX", 3),
        ("BX", 0),
        ("CX", 6),
        ("AY", 6),
        ("BY", 3),
        ("CY", 0),
        ("AZ", 0),    
        ("BZ", 6),  
        ("CZ", 3), 
    ]);   

    let win_strat = HashMap::from([
        ("X", "lose"),
        ("Y", "draw"),
        ("Z", "win"),
    ]);   

    let strategy_map =  HashMap::from([
        ("AX", 3), // Rock -> lose -> Scissors -> 3+0
        ("BX", 1), // Paper -> lose -> Rock -> 1+0
        ("CX", 2), // Scissors -> lose -> Paper -> 2+0
        ("AY", 4), // Rock -> draw -> Rock -> 1+3
        ("BY", 5), // Paper -> draw -> Paper -> 2+3
        ("CY", 6), // Scissors -> draw -> Scissors -> 3+3
        ("AZ", 8), // Rock -> win -> Paper -> 2+6 
        ("BZ", 9), // Paper -> win -> Scissors -> 3+6
        ("CZ", 7), // Scissors -> win -> Rock -> 1+6

    ]);

    let mut total_points: i32 = 0;
    let mut strategy_points: i32 = 0;
    
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./data/input.txt") {

        for line in lines {

            if let Ok(ip) = line {
                println!("Current Game {:?}", ip.replace(" ", ""));

                let my_hand = ip.chars().nth(0).unwrap().to_string();
                let other_hand = ip.chars().nth(2).unwrap().to_string();

                let my_points = points.get_key_value(&other_hand as &str);
                let game_points = wins_map.get_key_value(&ip.replace(" ", "") as &str);
                
                println!("My Points {:?}", my_points);
                println!("Game Points {:?}", game_points);

                total_points += my_points.unwrap().1 + game_points.unwrap().1;

                strategy_points += strategy_map.get_key_value(&ip.replace(" ", "") as &str).unwrap().1;
                }
            }
    }
    println!("Total Points {:?}", total_points);
    println!("Strategy Points {:?}", strategy_points);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}