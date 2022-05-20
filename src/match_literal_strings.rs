use regex::Regex;

pub fn _match_literals() -> bool {
    let waldo_is_hiding = "Somewhere Waldo is hiding in this text.";

    /* NOTE: This is case-sensitive */
    let regex = Regex::new(r"Waldo").unwrap();

    regex.is_match(waldo_is_hiding)
}

#[test]
fn should_return_true() {
    let result = _match_literals();

    assert_eq!(result, true);
}