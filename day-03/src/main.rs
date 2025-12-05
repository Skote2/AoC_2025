use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Pass input file as argument!");
        return;
    }
    let input_path = &args[1];

    println!("Input file \"{input_path}\"");

    let mut max_joltage: u64 = 0;
    let max_batteries = 12;
    if let Ok(lines) = read_lines(input_path) {
        for line_string in lines.map_while(Result::ok) {
            println!("{}", line_string);
            let mut line = line_string.as_str();

            let mut remaining_batteries = max_batteries;
            let mut answer:u64 = 0;
            while remaining_batteries > 0 {
                let max = get_max(&line, remaining_batteries);
                let digit_char = line.char_indices().nth(max).unwrap();
                let digit_val:u64 = digit_char.1.to_digit(10).unwrap().into();
                remaining_batteries-=1;
                answer += digit_val * 10_u64.pow(remaining_batteries.try_into().unwrap());
                //println!("answer: {answer}");

                line = &line[digit_char.0+1..];
            }

            println!("line_max: {answer}");
            max_joltage += answer;
        }
    } else {
        println!("failed to open file!");
        return;
    }
    println!("max_joltage: {max_joltage}");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_max(input:&str, chars_remaining:usize) -> usize {
    let mut max = '0';
    let mut pos = 0;
    for i in input.char_indices() {
        if i.0 == input.len()-chars_remaining+1 { break; }
        if max < i.1 {
            max = i.1;
            pos = i.0;
        }
    }
    return pos;
}
