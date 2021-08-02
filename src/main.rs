use std::result::Result;
use std::env;
use std::fs;
use std::process;

struct Expression (usize, String);

fn main() {
    match env::args().len() {
        1 => help(),
        2 => {
            let argument: String = env::args().last().unwrap();
            match argument.as_str() {
                "-h"|"--help" => {
                    return help();
                },
                _ => {}
            }
            match parse_expression(&argument) {
                Ok(v) => {
                    let Expression(count, delimiter) = v;
                    lorem(count, &delimiter);
                },
                Err(err) => return println!("Error: {}", err)
            };
        },
        _ => {
            println!("Invalid argument.");
            help();
        }
    }
}

fn parse_expression(expression: &str) -> Result<Expression, &str> {
    let input = expression.trim();
    if input.len() < 2 {
        return Err("Empty string");
    }

    let count: usize = match input[0..input.len()-1].parse() {
        Ok(n) => n,
        Err(_) => return Err("Invalid count")
    };
    let suffix = match input.chars().last() {
        Some(s) => s,
        None => return Err("Missing argument suffix")
    };
    let delimiter: &str = match suffix {
        'w' => " ",
        's' => ". ",
        'c' => "\0",
        _ => return Err("Invalid argument suffix")
    };

    Ok(Expression(count, String::from(delimiter)))
}

fn lorem(count: usize, delimiter: &str) {
    let text = fs::read_to_string("lorem.txt").unwrap();
    let mut read_count = count;
    let chunks: String = match delimiter {
        "\0" => {
            if text.len() < read_count {
                read_count = text.len();
            }
            String::from(&text[..read_count])
        },
        _ => {
            let chunks: Vec<&str> = text.split(delimiter).collect();
            if chunks.len() < read_count {
                read_count = chunks.len();
            }
            chunks[..read_count].join(delimiter)
        }
    };
    println!("{}", chunks)
}

fn help() {
    println!("Usage: lorem [count](w|s|c)\n");
    println!("Prints specified amount of words (w), sentenses (s) or characters(c) of lorem ipsum text.");
    println!("Example:\n lorem 2s\n lorem 4w\n lorem 128c");
    process::exit(1);
}
