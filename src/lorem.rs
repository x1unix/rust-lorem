const SPLIT_BY_CHAR: &str = "\0";

pub struct Expression (pub usize, pub String);

pub fn parse_expression(expression: &str) -> Result<Expression, &str> {
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
        'c' => SPLIT_BY_CHAR,
        _ => return Err("Invalid argument suffix")
    };

    Ok(Expression(count, String::from(delimiter)))
}

pub fn generate(text: &str, count: usize, delimiter: &str) -> String {
    // let text = fs::read_to_string("lorem.txt").unwrap();
    let mut read_count = count;
    let chunks: String = match delimiter {
        SPLIT_BY_CHAR => {
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
    chunks
}
