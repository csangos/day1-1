use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn load_from_file(file_path: &str) -> Vec<i32> {
    let file = File::open(file_path).expect("file wasn't found.");
    let reader = BufReader::new(file);

    let numbers: Vec<i32> = reader
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect();

    numbers
}

fn depths(numbers: Vec<i32>) {
    let mut total = 0;
    let mut last_number: i32 = 0;

    for number in numbers {
        if last_number > 0 {
            if number > last_number {
                total += 1;
            }
        }
        last_number = number;
    }
    println!("Total: {}", total);
}

fn main() {
    let numbers = load_from_file("src/depths.txt");
    depths(numbers); // 1-1: count the depths that are larger than the last one
}
