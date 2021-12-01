use std::fs;
use std::cmp::Ordering;

fn main() {
    let filename: String = String::from("../input");
    let input = read_input(&filename);
    
    let mut increases = 0;

    let sliding_windows = create_sliding_windows(input);
    for i in 1..sliding_windows.len() {
        let previous = &sliding_windows[i-1];
        let current = &sliding_windows[i];

        if check_larger(&previous.sum(), &current.sum()) {
            increases = increases+1;
        }
    }
    println!("Amount of increases = {}", increases);
}

fn read_input(filename: &String ) -> Vec<u32>
{
    let content = fs::read_to_string(filename)
    .expect("Something went wrong when reading the  file");
    let mut result: Vec<u32>  = Vec::new();
    for line in content.lines() {
        result.push(line.parse().unwrap());
    }
    result
}

fn check_larger(prev: &u32, current: &u32) -> bool
{
    match current.cmp(prev) {
        Ordering::Less => false,
        Ordering::Equal => false,
        Ordering::Greater => true
    }
}

fn create_sliding_windows(input: Vec<u32>) -> Vec<SlidingWindow>
{
    let mut result: Vec<SlidingWindow> = Vec::new();

    for i in 2..input.len() {
        let values = vec![
            input[i-2],
            input[i-1],
            input[i]
        ];

        result.push(
            SlidingWindow {
                values
            }
        );
    }

    result
}

#[derive(Debug)]
struct SlidingWindow {
    values: Vec<u32>,
}

impl SlidingWindow {
    fn sum(&self) -> u32 {
        let mut sum = 0;
        for v in self.values.iter() {
            sum = sum +v;
        }
        sum
    }
}