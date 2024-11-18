use std::io;

fn to_celsius(input: f64) -> f64 {
    (input - 32 as f64) * 5.0 / 9.0
}

fn to_fahrenheit(input: f64) -> f64 {
    input / (5.0 / 9.0) + 32.0
}

fn main() {
    // Convert temperatures between Fahrenheit and Celsius.

    // Formula: (F − 32) × 5/9 = C

    println!("Input a temperature with symbol (Fahrenheit or Celsius):");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(_) => {}
    }

    let input = input.trim();

    println!("Input is {input}. Performing conversion.");

    let cleaned_input = input.to_lowercase();

    let suffix ;
    let result_suffix ;
    let temp_conversion: fn(f64) -> f64;

    if cleaned_input.ends_with("c") {
        suffix = "c";
        result_suffix = "F";
        temp_conversion = to_fahrenheit
    } else if cleaned_input.ends_with("f") {
        suffix = "f";
        result_suffix = "C";
        temp_conversion = to_celsius
    } else {
        println!("Unable to convert input {input}.");
        return;
    }

    let cleaned_input = cleaned_input.trim_end_matches(suffix);

    let cleaned_input: f64 = cleaned_input.parse().expect("Invalid input: {input}.");

    let result = temp_conversion(cleaned_input);

    println!("Result: {result}{result_suffix}")
}
