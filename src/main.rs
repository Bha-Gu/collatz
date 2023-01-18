use std::io;

fn main() {
    let mut input_str = String::new();

    // Define input variable
    let input: u32;

    // Read user's input and parse it into a u32
    input = loop {
        println!("Enter the number: ");

        // Read input from standard input
        io::stdin()
            .read_line(&mut input_str)
            .expect("failed to read line");

        // Try to parse input as a u32
        match input_str.trim().parse() {
            // If successful, return the value of input
            Ok(num) => num,
            // If not successful, print an error message and continue the loop
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        }

        // If the input is 0, re-prompt the user and continue the loop
        if input == 0 {
            println!("Cannot use 0 as input");
            continue;
        } else {
            // If the input is not 0, exit the loop
            break input;
        }
    };

    // Print the first number in the sequence
    print!("{}", input);

    let mut current = input;
    while current != 1 {
        // If current is even, divide it by 2
        if current % 2 == 0 {
            current /= 2;
        } else {
            // If current is odd, multiply it by 3 and add 1
            current = current * 3 + 1;
        }
        print!(", {}", current);
    }
}
