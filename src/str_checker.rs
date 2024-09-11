pub fn is_number(test: &str) -> bool {
    for c in test.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    !test.is_empty()
}

pub fn is_word(test: &str) -> bool {
    for c in test.chars() {
        if !c.is_alphabetic() {
            return false;
        }
    }
    !test.is_empty()
}

pub fn is_alphanumeric(test: &str) -> bool {
    for c in test.chars() {
        if !c.is_alphanumeric() {
            return false;
        }
    }
    !test.is_empty()
}
