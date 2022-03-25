use untitled2::capitalize;

#[test]
fn check1 () {
    assert_eq!(capitalize
        ("ciao"),"Ciao");
}

#[test]
fn check_more_than_one_word(){
    assert_eq!(capitalize("a b"), "A B");
}

#[test]
fn check_one_word_without_space(){
    assert_eq!(capitalize("ab"), "Ab");
}

#[test]
fn check_accented_character(){
    assert_eq!(capitalize("è"), "È");
}

#[test]
fn check_is_empty(){
    assert_eq!(capitalize(""),"");
}

#[test]
fn check_more_spaces(){
    assert_eq!(capitalize("a b   c"), "A B   C");
}