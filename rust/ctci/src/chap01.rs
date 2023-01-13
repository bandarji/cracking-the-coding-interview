use std::collections::HashSet;

static STR_UNIQUE: &str = "uncopyrightable";
static STR_NONUNIQUE: &str = "bookkeeper";

static STR_PERM_1: &str = "angered";
static STR_PERM_2: &str = "enraged";
static STR_NOT_PERM_LEN: &str = "upset";
static STR_NOT_PERM_FAIL: &str = "annoyed";

// q1.1: Does string have unique characters, using a set
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

// q1.1: Does string have unique characters using bits
// (no external data structures)
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

// q1.2: String a permutation of another
// Sorting the characters of the string and comparing vectors
fn is_permutation_sort(string1: &str, string2: &str) -> bool {
    if string1.len() != string2.len() {
        return false;
    }
    sort_chars(string1) == sort_chars(string2)
}

// q1.2 helper function for sorting the string
fn sort_chars(string: &str) -> String {
    let mut chars: Vec<char> = string.chars().collect();
    chars.sort();
    chars.into_iter().collect::<String>()
}

// q1.2: String a permutation of another
// Array with character counts, presuming ASCII
fn is_permutation_counts(string1: &str, string2: &str) -> bool {
    let mut counts: [i8; 128] = [0; 128];
    if string1.len() != string2.len() {
        return false;
    }
    for c in string1.chars() {
        let index = c as i8;
        counts[index as usize] += 1;
    }
    for c in string2.chars() {
        let index: i8 = c as i8;
        counts[index as usize] -= 1;
        if counts[index as usize] < 0 {
            return false;
        }
    }
    true
}

pub fn tests01() {
    println!("1.1");
    println!("  Testing if '{}' has unique characters, with a set: {}", STR_UNIQUE, is_unique_chars_set(STR_UNIQUE));
    println!("  Testing if '{}' has unique characters, with bits: {}", STR_UNIQUE, is_unique_chars_bits(STR_UNIQUE));
    println!("  Testing if '{}' has non-unique characters, with a set: {}", STR_NONUNIQUE, is_unique_chars_set(STR_NONUNIQUE));
    println!("  Testing if '{}' has non-unique characters, with bits: {}", STR_NONUNIQUE, is_unique_chars_bits(STR_NONUNIQUE));
    println!("1.2");
    println!("  Testing if '{}' is a permutation of '{}', by sorting: {}", STR_PERM_1, STR_PERM_2, is_permutation_sort(STR_PERM_1, STR_PERM_2));
    println!("  Testing if '{}' is a permutation of '{}', by sorting: {}", STR_PERM_1, STR_NOT_PERM_LEN, is_permutation_sort(STR_PERM_1, STR_NOT_PERM_LEN));
    println!("  Testing if '{}' is a permutation of '{}', by sorting: {}", STR_PERM_1, STR_NOT_PERM_FAIL, is_permutation_sort(STR_PERM_1, STR_NOT_PERM_FAIL));
    println!("  Testing if '{}' is a permutation of '{}', by counting: {}", STR_PERM_1, STR_PERM_2, is_permutation_counts(STR_PERM_1, STR_PERM_2));
    println!("  Testing if '{}' is a permutation of '{}', by counting: {}", STR_PERM_1, STR_NOT_PERM_LEN, is_permutation_counts(STR_PERM_1, STR_NOT_PERM_LEN));
    println!("  Testing if '{}' is a permutation of '{}', by counting: {}", STR_PERM_1, STR_NOT_PERM_FAIL, is_permutation_counts(STR_PERM_1, STR_NOT_PERM_FAIL));
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

#[test]
fn test_is_permutation_sort() {
    assert_eq!(is_permutation_sort(STR_PERM_1, STR_PERM_2), true);
    assert_eq!(is_permutation_sort(STR_PERM_1, STR_NOT_PERM_LEN), false);
    assert_eq!(is_permutation_sort(STR_PERM_1, STR_NOT_PERM_FAIL), false);
}

#[test]
fn test_is_permutation_counts() {
    assert_eq!(is_permutation_counts(STR_PERM_1, STR_PERM_2), true);
    assert_eq!(is_permutation_counts(STR_PERM_1, STR_NOT_PERM_LEN), false);
    assert_eq!(is_permutation_counts(STR_PERM_1, STR_NOT_PERM_FAIL), false);
}
