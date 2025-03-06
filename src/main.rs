use regex::Regex;

fn apply_operation(pattern: Regex, mut expression: String, operation: &str) -> String {
    if operation.is_empty() {
        return "".to_string();
    }
    loop {
        let caps = pattern.captures(expression.as_str());
        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let caps_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let res = match operation {
            "+" => left_value + right_value,
            "-" => left_value - right_value,
            "*" => left_value * right_value,
            "/" => left_value / right_value,
            _ => 0,
        };
        expression = expression.replace(caps_expression, &res.to_string());
    }
    return expression;
}

fn main() {
    // Regex
    let re_mult: Regex = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let re_div: Regex = Regex::new(r"(\d+)\s?\/\s?(\d+)").unwrap();
    let re_add: Regex = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_less: Regex = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();

    // User input
    let mut user_expression: String = String::new();
    println!("Insert your math expression:");
    std::io::stdin().read_line(&mut user_expression).unwrap();

    // Apply operations
    user_expression = apply_operation(re_mult, user_expression, "*");
    user_expression = apply_operation(re_div, user_expression, "/");
    user_expression = apply_operation(re_add, user_expression, "+");
    user_expression = apply_operation(re_less, user_expression, "-");

    // Show results
    println!("Result: {}", user_expression);
}
