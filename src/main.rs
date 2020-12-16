use std::io;

fn main() {
    println!("Search:");

    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("bytes: {}", n);
            println!("input: {}", input);
        }
        Err(error) => println!("error: {}", error),
    }

    println!("You inputted: {}", input.trim());
}
