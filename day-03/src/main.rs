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

    let mut max_joltage = 0;
    if let Ok(lines) = read_lines(input_path) {
        for line_String in lines.map_while(Result::ok) {
            println!("{}", line_String);
            let line = line_String.as_str();

            let max_pos = get_max(&line, true);
            let first_char = line.char_indices().nth(max_pos).unwrap();
            //println!("1stmax: ({}, {})", first_char.0, first_char.1);

            let remaining_line = &line[first_char.0+1..];
            //println!("remaining_line: {remaining_line}");
            let second_max = get_max(remaining_line, false);
            let second_char = remaining_line.char_indices().nth(second_max).unwrap();
            //println!("2ndmax: ({}, {})", second_char.0, second_char.1);

            let answer = first_char.1.to_digit(10).unwrap()*10 + second_char.1.to_digit(10).unwrap(); 
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

fn get_max(input:&str, skip_last:bool) -> usize {
    let mut max = '0';
    let mut pos = 0;
    for i in input.char_indices() {
        if skip_last && i.0 == input.len()-1 { break; }
        if max < i.1 {
            max = i.1;
            pos = i.0;
        }
    }
    return pos;
}
