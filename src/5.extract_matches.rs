use regex::Regex;

pub fn _extract_match() {
    let extract_str = "Extract the word 'coding' from this string.";
    let regex = Regex::new(r"coding").unwrap();

    let result = regex.captures(extract_str).unwrap();

    let result = result.get(0).map_or("", |arg| arg.as_str());

    println!("{}", result);
}