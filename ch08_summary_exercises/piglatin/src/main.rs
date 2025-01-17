fn main() {
    // Convert strings to pig latin.
    // The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay.
    // Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
    // Keep in mind the details about UTF-8 encoding!

    let input = "first apple";

    let mut result = String::new();
    for word in input.split_whitespace() {
        let mut chars = word.chars();
        let first = chars.next().unwrap();

        if result.len() > 0 {
            result.push(' ');
        }
        result += &match first {
            'a' | 'e' | 'i' | 'o' | 'u' => format!("{word}-hay"),
            _ => {
                let trimmed = chars.as_str();
                format!("{trimmed}-{first}ay")
            }
        }
    }

    println!("input: {} result: {}", input, result);
}
