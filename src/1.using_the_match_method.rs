use regex::Regex;

pub fn _use_match() -> bool {
    let sentence = "The dog chased the cat.";
    let regex = Regex::new(r"the").unwrap();

    // Rust is_match method is similar to Javascript test method.
    let result = regex.is_match(sentence);

    result
}

#[test]
fn should_return_true() {
    let result = _use_match();

    assert_eq!(result, true)
}