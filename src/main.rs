use std::env;
use std::fs;
use std::process;
use lorem::{generate, parse_expression, Expression};

mod lorem;

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
                Ok(v) => print_lorem(v),
                Err(err) => println!("Error: {}", err)
            };
        },
        _ => {
            println!("Invalid argument.");
            help();
        }
    }
}

fn print_lorem(expr: Expression) {
    let text = fs::read_to_string("lorem.txt").unwrap();
    let Expression(count, delimiter) = expr;
    let result = generate(&text, count, &delimiter);
    println!("{}", result);
}

fn help() {
    println!("Usage: lorem [count](w|s|c)\n");
    println!("Prints specified amount of words (w), sentenses (s) or characters(c) of lorem ipsum text.");
    println!("Example:\n lorem 2s\n lorem 4w\n lorem 128c");
    process::exit(1);
}
