use regex::Regex;

fn main() {
    // Regex
    let re_add: Regex = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    // User input
    let mut user_expression: String = String::new();
    println!("Insert your math expression:");
    std::io::stdin().read_line(&mut user_expression).unwrap();
    // Apply operations
    let caps = re_add.captures(&mut user_expression).unwrap();
    let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
    let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

    // Show results
    println!("Result: {}", left_value + right_value);
}
