use std::collections::BTreeMap;

fn main() {
    println!("Hello, aoc!");
    // Read input file from arguments
    let args: Vec<String> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1]).expect("Error reading input file!");
    let sum: i32 = decode_input(&input).iter().sum();
    println!("Numbers: {:?}", sum);
}

fn decode_input(input: &str) -> Vec<i32> {
    input.lines().map(get_number_from_line).collect()
}

fn get_number_from_line(line: &str) -> i32 {
    // BTree for word numbers like 'one', 'two', 'three' up to 'nine'
    let mut word_tree = BTreeMap::new();
    word_tree.insert("one", 1);
    word_tree.insert("two", 2);
    word_tree.insert("three", 3);
    word_tree.insert("four", 4);
    word_tree.insert("five", 5);
    word_tree.insert("six", 6);
    word_tree.insert("seven", 7);
    word_tree.insert("eight", 8);
    word_tree.insert("nine", 9);
    let mut digit = vec![];
    let mut peekable_chars = line.chars().peekable();
    while let Some(c) = peekable_chars.next() {
        if c.is_ascii_digit() {
            digit.push(c);
        }
        // If it is a word, get the number from the BTree otherwise skip
        if c.is_ascii_alphabetic() {
            // Match candidate words
            let mut word = String::new();
            word.push(c);
            while let Some(&c) = peekable_chars.peek() {
                if c.is_ascii_alphabetic() {
                    word.push(c);
                    peekable_chars.next();
                } else {
                    break;
                }
            }
        }
    }
    // Take only first and last digit
    let number = format!("{}{}", digit[0], digit[digit.len() - 1]);
    number.parse::<i32>().unwrap()
}
