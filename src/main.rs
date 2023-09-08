use std::collections::HashMap;

fn product(x: u32, y: u32) -> u32 {
    if y == 1 { x }
    else { x + product(x, y-1) }
}

fn fibonnacci(x: u32) -> u32
    { if x == 0 || x == 1 { 1 } else { fibonnacci(x-1) + fibonnacci(x - 2) } }

fn division(x: u32, y: u32) -> u32 {
    if y == 0 { 1 }
    else { x - division(x, y-1) }
}

fn count_vowel(text: String) -> HashMap<char, u32> {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut vowels_map: HashMap<char, u32> = HashMap::new();
    for str in text.chars() {
        if !(vowels_map.contains_key(&str)) && vowels.contains(&str) {
            vowels_map.insert(str, 1);
        } else {
            vowels_map.entry(str).and_modify(|value| *value+=1);
        }
    }
    return vowels_map;
}

fn main() {
    let text: String = "Gets the given key's corresponding entry in the map for in-place manipulation".to_string();
    let response = count_vowel(text.to_lowercase());
    for i in response.iter() {
        println!("key: {} --- value: {}", i.0, i.1);
    }
}
