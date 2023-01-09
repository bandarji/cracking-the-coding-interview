use std::collections::HashSet;

static STR_UNIQUE: &str = "uncopyrightable";
static STR_NONUNIQUE: &str = "bookkeeper";

pub fn tests01() {
    println!("Testing if '{}' has unique characters, with a set: {}", STR_UNIQUE, is_unique_chars_set(STR_UNIQUE));
    println!("Testing if '{}' has unique characters, with bits: {}", STR_UNIQUE, is_unique_chars_bits(STR_UNIQUE));
    println!("Testing if '{}' has non-unique characters, with a set: {}", STR_NONUNIQUE, is_unique_chars_set(STR_NONUNIQUE));
    println!("Testing if '{}' has non-unique characters, with bits: {}", STR_NONUNIQUE, is_unique_chars_bits(STR_NONUNIQUE));
}

#[test]
fn test_unique() {
    assert_eq!(is_unique_chars_set(STR_UNIQUE), true);
    assert_eq!(is_unique_chars_bits(STR_UNIQUE), true);
}

#[test]
fn test_not_unique() {
    assert_eq!(is_unique_chars_set(STR_NONUNIQUE), false);
    assert_eq!(is_unique_chars_bits(STR_NONUNIQUE), false);
}

fn is_unique_chars_set(string: &str) -> bool {
    let mut chars: HashSet<char> = HashSet::new();
    for char in string.chars() {
        if chars.contains(&char) {
            return false;
        }
        chars.insert(char);
    }
    true
}

fn is_unique_chars_bits(string: &str) -> bool {
    let mut bits: i32 = 0;
    let char_a: i16 = 'a' as i16;
    for char in string.chars() {
        let mut char_int: i16 = char as i16;
        char_int -= char_a;
        if (1 << char_int) & bits != 0 {
            return false;
        }
        bits |= 1 << char_int;
    }
    true
}
