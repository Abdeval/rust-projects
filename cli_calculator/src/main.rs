use std::io::stdin;

pub fn add(l: f64, r: f64) -> f64 {
    l + r
}

pub fn multiply(l: f64, r: f64) -> f64 {
    l * r
}

pub fn substract(l: f64, r: f64) -> f64 {
    l - r
}

pub fn divide(l: f64, r: f64) -> f64 {
    if r == 0.0 {
        panic!("Division by 0 is impossible");
    }
    l / r
}

fn normal_calculation(tokens: &Vec<&str>) -> f64 {
    let mut result = tokens[0].parse::<f64>().unwrap();
    let mut i = 1;
    while i < tokens.len() {
        let op = tokens[i].parse::<char>().unwrap();
        let num = tokens[i + 1].parse::<f64>().unwrap();
        match op {
            '+' => result += num,
            '-' => result -= num,
            '*' => result *= num,
            '/' => result /= num,
            _ => panic!("Operator is not valid"),
        }
        i += 2;
    }

    result
}

// todo: advanced calculation
pub fn advanced_calcualtion() -> f64 {
    0.0
}

fn main() {
    let mut input = String::new();
    println!("Enter the numbers like this format 1 + 4:");
    stdin().read_line(&mut input).unwrap();
    let tokens: Vec<&str> = input.trim().split_whitespace().collect();
    let result = normal_calculation(&tokens);
    println!("Result: {}", result);
}
