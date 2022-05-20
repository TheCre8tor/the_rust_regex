use regex::Regex;

pub fn _ignore_case() {
    let my_string = "freeCodeCamp";

    // javascript -> /freecodecamp/i
    // Rust -> r"(?i)freecodecamp"
    /* The (?i) tells Regex to ignore case in Rust */
    let regex = Regex::new(r"(?i)freecodecamp").unwrap();

    let result = regex.captures(my_string).unwrap();

    let result = match result.get(0) {
        Some(arg) => arg.as_str(),
        None => ""
    };

    println!("{:?}", result)
}