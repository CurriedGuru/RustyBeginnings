use std::io;

fn main() {
    loop {
        println!("To print the Nth Fibonacci number, enter the value of N");
        println!("(example: for the first Fibonacci number, enter 1)");
        let n = match read_from_stdin_u32() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}\n\n", err);
                continue
            },
        }; 
        if n < 1 {
            println!("[Error]Please enter a valid, positive number.\n\n");
            continue;
        }

        if n > 185 {
            println!("[Error]Sorry, Rust data types can hold up to the 185th Fibonacci number only.\n\n");
            continue;
        }
        println!("The {}{} fibonacci number is {}\n\n", n, suffix(n), fibonacci(n));
    }
}

fn suffix(n: u32) -> String {
    if n == 11 || n == 12 || n == 13 {
        "th".to_string();
    }

    return match n % 10 {
        1 => "st".to_string(),
        2 => "nd".to_string(),
        3 => "rd".to_string(),
        _ => "th".to_string(),
    };
}

fn fibonacci(n: u32) -> u128 {
    if n == 1 || n == 2 {
        return 1;
    }
    let mut counter = 2;
    let mut current: u128 = 1;
    let mut previous: u128 = 1;

    return loop {
        let next = current + previous;
        previous = current;
        current = next;
        counter = counter + 1;

        if counter == n {
            break current;
        }
    };
}

fn read_string_from_stdin() -> io::Result<String> {
    let mut value = String::new();
    return match io::stdin().read_line(&mut value) {
        Ok(_) => Ok(value.trim().to_string()),
        Err(err) => Err(err),
    };
}

fn read_from_stdin_u32() -> Result<u32, String> {
    match read_string_from_stdin() {
        Ok(str) => {
            match str.parse() {
                Ok(num) => return Ok(num),
                Err(_) =>  return Err(String::from("Unable to parse as an unsigned integer(u32)")),
            };
        },
        Err(_) => return Err(String::from("Unable to parse integer")),
    };
}
