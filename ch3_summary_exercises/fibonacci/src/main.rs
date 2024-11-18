use std::io;

fn fib(nth_term: u64) -> (u64, u64) {
    if nth_term < 2 {
        return (nth_term, nth_term);
    }

    let mut last_term:u64 = 0;
    let mut current_term = 1;

    for term in 2..u64::MAX {

        let last_term_holder = current_term;

        match last_term.checked_add(current_term) {
            Some(result) => current_term = result,
            None => return (term-1, current_term),
        }

        if nth_term == term.try_into().unwrap() {
            return (term, current_term);
        }

        last_term = last_term_holder;
    };

    panic!("Could not find nth term!")
}

fn main() {
    // Generate the nth Fibonacci number.

    println!("Generate the nth Fibonacci number. N?");

    let mut nth_term = String::new();

    io::stdin()
        .read_line(&mut nth_term)
        .expect("Failed to read nth term from stdin");

    println!("Entered {nth_term}. Finding...");

    let nth_term: u64 = nth_term.trim()
        .parse()
        .expect("Could not parse {nth_term} as an nth term.");

    let (term, result) = fib(nth_term);

    println!("{term}th term is {result}");
}
