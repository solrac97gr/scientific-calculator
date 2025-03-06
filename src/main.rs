use regex::Regex;

fn main() {
    let regex_patter: &str = r"(?-u:\w){1000}";
    assert!(Regex::new(regex_patter).is_ok());
}
