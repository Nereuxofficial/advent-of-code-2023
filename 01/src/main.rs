use std::collections::BTreeMap;

fn main() {
    println!("Hello, aoc!");
    // Read input file from arguments
    let args: Vec<String> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1]).expect("Error reading input file!");
    let sum: i32 = decode_input(&input).iter().sum();
}

fn decode_input(input: &str) -> Vec<i32> {
    input.lines().map(get_number_from_line).collect()
}

fn get_btreemap() -> BTreeMap<&'static str, char> {
    // BTree for word numbers like 'one', 'two', 'three' up to 'nine'
    let mut word_tree = BTreeMap::new();
    word_tree.insert("one", '1');
    word_tree.insert("two", '2');
    word_tree.insert("three", '3');
    word_tree.insert("four", '4');
    word_tree.insert("five", '5');
    word_tree.insert("six", '6');
    word_tree.insert("seven", '7');
    word_tree.insert("eight", '8');
    word_tree.insert("nine", '9');
    word_tree
}

fn get_number_from_line(line: &str) -> i32 {
    let mut digit = vec![];
    let word_tree = get_btreemap();
    for c in 0..line.len(){
        // Check if char is a digit
        if line[c..=c].parse::<i32>().is_ok() {
            digit.push(line[c..=c].parse::<i32>().unwrap());
        } else {
            // Check if char is a letter
            if line[c..=c].parse::<char>().is_ok() {
                // Check if word matches a number
                if let Some(number) = matches_word(&line[c..], &word_tree) {
                    digit.push(number.to_digit(10).unwrap() as i32);
                }
            }
        }
    }
    // Take only first and last digit
    let number = format!("{}{}", digit[0], digit[digit.len() - 1]);
    //println!("{} -> {}", line, number);
    number.parse::<i32>().unwrap()
}

fn matches_word(word: &str, word_tree: &BTreeMap<&str, char>) -> Option<char> {
    // Try to find a match in the word_tree starting at the first letter, but we the word could be shorter than that in the tree
    for i in 2..word.len() {
        let candidate = &word[0..=i];
        if let Some(&number) = word_tree.get(candidate) {
            return Some(number);
        }
    }
    None
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_short_2() {
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;
        let numbers = decode_input(input);
        assert_eq!(numbers, vec![29, 83, 13, 24, 42, 14, 76]);
    }

    #[test]
    fn test_eighty_three() {
        let input = r#"eightwothree"#;
        let number = get_number_from_line(input);
        assert_eq!(number, 83);
    }
    #[test]
    fn test_matches(){
        let word_tree = get_btreemap();
        assert_eq!(matches_word("one", &word_tree), Some('1'));
    }
}