use regex::Regex;

fn apply_operation(pattern: Regex, mut expression: String, operation: &str) -> String {
    if operation.is_empty() {
        return String::new();
    }

    while let Some(caps) = pattern.captures(&expression) {
        let caps_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let res = match operation {
            "+" => left_value + right_value,
            "-" => left_value - right_value,
            "*" => left_value * right_value,
            "/" => {
                if right_value == 0 {
                    panic!("Division by zero");
                }
                left_value / right_value
            }
            _ => 0,
        };

        expression = expression.replacen(caps_expression, &res.to_string(), 1);
    }

    expression
}

fn main() {
    // User input
    let mut user_expression: String = String::new();
    println!("Insert your math expression:");
    std::io::stdin().read_line(&mut user_expression).unwrap();

    let operation_herarchy = ["*", "/", "+", "-"];

    // Apply operations
    for operation in operation_herarchy {
        let regex_to_use = get_regex_for_operation(operation).unwrap();
        user_expression = apply_operation(regex_to_use, user_expression, operation);
    }

    // Show results
    println!("Result: {}", user_expression);
}

fn get_regex_for_operation(operation: &str) -> Option<Regex> {
    // Compile regex patterns once
    let re_mult = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?\/\s?(\d+)").unwrap();
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_less = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();

    // Return the appropriate regex based on the operation
    match operation {
        "+" => Some(re_add),
        "-" => Some(re_less),
        "*" => Some(re_mult),
        "/" => Some(re_div),
        _ => None, // Return None for invalid operations
    }
}