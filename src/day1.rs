use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

pub fn day1()->(i32,i32) {
    // Specify the file path
    let file_path = "./data/day1.txt";
    // Arrays to store the numbers
    let mut array1 = Vec::new();
    let mut array2 = Vec::new();
    let mut map: HashMap<i32, i32> = HashMap::new();

    // Open the file
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(line) = line {
                // Parse each line into two numbers
                let numbers: Vec<i32> = line
                    .split_whitespace()
                    .filter_map(|x| x.parse::<i32>().ok())
                    .collect();

                if numbers.len() == 2 {
                    array1.push(numbers[0]);
                    array2.push(numbers[1]);
                    map.entry(numbers[1]).and_modify(|v| *v+=1).or_insert(1);
                } else {
                    eprintln!("Line does not contain exactly two numbers: {}", line);
                }
            }
        }
    } else {
        eprintln!("Could not open the file: {}", file_path);
    }

    // Sort the arrays
    array1.sort_unstable();
    array2.sort_unstable();

    let mut distance = 0;
    let mut similarity=0;
    for (index, num) in array1.iter().enumerate() {
        distance += (array2.get(index).unwrap_or(&0)-num).abs();
        similarity += num*map.get(&num).unwrap_or(&0);
    }
    return (distance,similarity);
}

// A function to return an iterator over lines of a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
