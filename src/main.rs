mod lorem;

use std::env;
use std::process;
use lorem::{generate, parse_expression, Expression};

static LOREM_IPSUM_TEXT: &'static str = include_str!("../res/lorem.txt");

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
    let Expression(count, delimiter) = expr;
    let result = generate(LOREM_IPSUM_TEXT, count, &delimiter);
    println!("{}", result);
}

fn help() {
    println!("Usage: lorem [count](w|s|c)\n");
    println!("Prints specified amount of words (w), sentenses (s) or characters(c) of lorem ipsum text.");
    println!("Example:\n lorem 2s\n lorem 4w\n lorem 128c");
    process::exit(1);
}
