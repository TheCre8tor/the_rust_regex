use regex::{Regex, Captures};

pub fn _matches() {
    let my_string = "Repeat Repeat Repeat";

    let regex = Regex::new(r"Repeat").unwrap();

    let result: Vec<Captures> = regex.captures_iter(my_string).collect();

    let mut extract: Vec<&str> = Vec::new();

    for item in 0..result.len() {
        let item = result[item].get(0).map_or("", |item| item.as_str());
        extract.push(item);
    }

    println!("{:?}", extract);
}