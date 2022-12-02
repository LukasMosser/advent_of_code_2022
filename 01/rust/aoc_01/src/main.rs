use std::fs::File;
use std::collections::VecDeque;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut q = VecDeque::from([]);

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./data/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let mut max_val: i64 = -1;
        let mut temp: i64 = 0;

        for line in lines {

            if let Ok(ip) = line {
                if !ip.is_empty() {
                    let val: i64 = ip.parse().unwrap();
                    temp += val;
                }
                else {
                    q.push_back(temp);
                    if max_val <= temp {
                        max_val = temp;
                    }
                    temp = 0;
                } 
            }

        }
        println!("Largest calories {}", max_val);
        q.make_contiguous().sort();
        let sum: i64 = q.iter().rev().take(3).sum();

        println!("Sum of top 3 elves {:?}", sum);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}