use std::fs::File;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;
use std::ops::{Bound, RangeBounds};
use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let points = HashMap::<&str, i32>::from([
        ("a", 1),
        ("b", 2),
        ("c", 3),
        ("d", 4),
        ("e", 5),
        ("f", 6),
        ("g", 7),
        ("h", 8),
        ("i", 9),
        ("j", 10),
        ("k", 11),
        ("l", 12),
        ("m", 13),
        ("n", 14),
        ("o", 15),
        ("p", 16),
        ("q", 17),
        ("r", 18),
        ("s", 19),
        ("t", 20),
        ("u", 21),
        ("v", 22),
        ("w", 23),
        ("x", 24),
        ("y", 25),
        ("z", 26),
        ("A", 27),
        ("B", 28),
        ("C", 29),
        ("D", 30),
        ("E", 31),
        ("F", 32),
        ("G", 33),
        ("H", 34),
        ("I", 35),
        ("J", 36),
        ("K", 37),
        ("L", 38),
        ("M", 39),
        ("N", 40),
        ("O", 41),
        ("P", 42),
        ("Q", 43),
        ("R", 44),
        ("S", 45),
        ("T", 46),
        ("U", 47),
        ("V", 48),
        ("W", 49),
        ("X", 50),
        ("Y", 51),
        ("Z", 52),
    ]);
    
    let mut total_points: i32 = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./data/input.txt") {

        for line in lines {

            if let Ok(ip) = line {
                let length: usize = ip.chars().count();
                let slice_first = ip.substring(0, length/2);
                let slice_second = ip.substring(length/2, length);
                //println!("First Slice {:?}", slice_first);
                //println!("Second Slice {:?}", slice_second);
                //println!("{:?}", share_char(slice_first, slice_second));

                let iter = slice_first.chars().filter(|c| slice_second.contains(*c));
                
                let mut v: Vec<char> = Vec::new();
                for itr1 in slice_first.chars() {
                    for itr2 in slice_second.chars() {
                        // println!("{}, {}", itr1, itr2);
                        if itr1 == itr2 {
                            v.push(itr1);
                        }
                    }
                }

                let hash_set: HashSet<char> = v.into_iter().collect();

                for c in hash_set {
                    total_points += points.get_key_value(&c.to_string() as &str).unwrap().1;
                }
     
            }
        }   
    }

    let mut total_points_group: i32 = 0;
    if let Ok(lines) = read_lines("./data/input.txt") {
        for (a, b, c) in lines.tuples() {
            let a_: String = a.unwrap();
            let b_: String = b.unwrap();
            let c_: String = c.unwrap();

            let mut v: Vec<char> = Vec::new();

            for itr1 in a_.chars() {
                for itr2 in b_.chars() {
                    for itr3 in c_.chars() {
                        if (itr1 == itr2) && (itr2 == itr3) {
                            v.push(itr1);
                        }
                    }
                }
            }

            let hash_set: HashSet<char> = v.into_iter().collect();
            
            for chars in hash_set {
                
                total_points_group += points.get_key_value(&chars.to_string() as &str).unwrap().1;
            }


        }
    }
    println!("{:?}", total_points);
    println!("{:?}", total_points_group);
}


fn share_char(a: &str, b: &str) -> bool {
    // fill the set with the characters from the shorter string
    let set: HashSet<char> = a.chars().collect();

    b.chars().any(|c| set.contains(&c))
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}