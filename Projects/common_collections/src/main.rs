use std::collections::HashMap;

fn main() {
    let intergers = [1, 2, 3, 4, 5, 6, 1];
    let intergers_vec = intergers.to_vec();

    print_median(&intergers_vec);
    let mode = print_mode(&intergers);
    println!("{:?}", intergers);
    println!("{mode}");
    let string = "hej ej";
    let pig_string = convert_to_pig_latin(string);
    println!("{pig_string}");
}

fn print_median(intergers_vec: &Vec<i32>) {
    let mut copy_vec = intergers_vec.clone();
    copy_vec.sort();
    let median: f32;
    if copy_vec.len() % 2 == 0 {
        let middle_index = copy_vec.len() / 2;
        median = (copy_vec[middle_index] + copy_vec[middle_index - 1]) as f32 / 2.0;
    } else {
        median = copy_vec[copy_vec.len() / 2] as f32;
    }

    println!("{median}");
}

fn print_mode(intergers: &[i32]) -> i32 {
    let mut value_amount_hash: HashMap<i32, i32> = HashMap::new();
    for i in intergers.iter() {
        *value_amount_hash.entry(*i).or_insert(0) += 1;
    }
    *value_amount_hash
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
        .unwrap()
}

fn convert_to_pig_latin(string: &str) -> String {
    let mut pig_latin_string = String::from("");
    for s in string.split_whitespace() {
        if is_vowel(&s.chars().next().unwrap()) {
            pig_latin_string.push_str(&s[1..]);
            pig_latin_string.push_str(&s[0..=0]);
            pig_latin_string.push_str("ay ")
        } else {
            pig_latin_string.push_str(s);
            pig_latin_string.push_str("hay ");
        }
    }
    pig_latin_string
}

fn is_vowel(c: &char) -> bool {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    for v in vowels.iter() {
        if c == v {
            return true;
        }
    }
    false
}
