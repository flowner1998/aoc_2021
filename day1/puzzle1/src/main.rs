use std::fs;
use std::cmp::Ordering;

fn main() {
    let filename: String = String::from("./input");
    let input = read_input(&filename);
    
    let mut increases = 0;

    for (index, value) in input.iter().enumerate() {
        if index == 0 {
            continue;
        }

        let prev = input.get(index-1).unwrap();

        if check_larger(prev, value) {
            increases+=1;
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
