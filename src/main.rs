use regex::Regex;

fn main() {
    // Regex
    let re_add: Regex = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    // User input
    let mut user_expression: String = String::new();
    println!("Insert your math expression:");
    std::io::stdin().read_line(&mut user_expression).unwrap();
    // Apply operations
    loop {
        let caps = re_add.captures(user_expression.as_str());
        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let caps_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let sum = left_value + right_value;
        user_expression = user_expression.replace(caps_expression, &sum.to_string());
    }
    // Show results
    println!("Result: {}", user_expression);
}
