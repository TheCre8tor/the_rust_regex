use regex::Regex;

pub fn _or_operator() -> bool {
    /* NOTE: Regex has an (or operator) which is a single 
       pipe (|) character */

    let pet_string = "James has a pet cat.";

    /* This is going to match any of this four words,
       and since one of this word is in the sentence,
       it will return true */
    let pet_regex = Regex::new(r"dog|cat|bird|fish").unwrap();
    let result = pet_regex.is_match(pet_string);

    result
}


#[test]
fn should_return_true_if_contained() {
    let result = _or_operator();

    assert_eq!(result, true);
}