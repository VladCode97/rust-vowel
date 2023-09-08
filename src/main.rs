use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_vowel(text: String) -> HashMap<char, u32> {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut vowels_map: HashMap<char, u32> = HashMap::new();
    for str in text.chars() {
        if !(vowels_map.contains_key(&str)) && vowels.contains(&str) {
            vowels_map.insert(str, 1);
        } else {
            vowels_map.entry(str).and_modify(|value| *value += 1);
        }
    }
    return vowels_map;
}

fn open_file(path: String) -> File {
    File::open(path).expect("File not found")
}

fn read_line_by_line(file: File) -> std::io::Result<String> {
    let buff_file = BufReader::new(file);
    let mut text: String = String::new();
    for line in buff_file.lines() {
        text += &line?;
    }
    Ok(text)
}

fn main() {
    let file = open_file("text.txt".to_string());
    let result_line_by_line = read_line_by_line(file);
    let text = result_line_by_line.unwrap().to_lowercase();
    for i in count_vowel(text).iter() {
        println!("key: {} --- value: {}", i.0, i.1);
    }
}
