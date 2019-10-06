use std::io;

fn main() {

    loop {
        println!("What conversion would you like to do?");
        println!("(fahrenheit to celsius: 1, celsius to fahrenheit: 2)");

        let choice = match read_string_from_stdin() {
            Ok(str) => {
                if str == "1" || str == "2" {
                   str 
                } else {
                    println!("You entered an invalid choice. Try again!\n\n");
                    continue
                }
            },

            Err(_) => {
                println!("You entered an invalid choice. Try again!\n\n");
                continue
            },
        };

        match choice.as_ref() {
            "1" => convert_f2c(),
            "2" => convert_c2f(),
            _ => continue 
        }
    }
}

fn convert_f2c() {
    println!("Enter the temperature in Fahrenheit:");
    match read_from_stdin_f64() {
        Ok(fahrenheit) => println!("{}F = {}C\n\n", fahrenheit, (fahrenheit-32.0)*5.0/9.0),
        Err(_) => println!("Invalid number entered. Try again!\n\n"),
    };
}

fn convert_c2f() {
    println!("Enter the temperature in Celsius:");
    match read_from_stdin_f64() {
        Ok(celsius) => println!("{}C = {}F\n\n", celsius, (celsius*9.0/5.0)+32.0),
        Err(_) => println!("Invalid number entered. Try again!\n\n"),
    };
}

fn read_string_from_stdin() -> io::Result<String> {
    let mut value = String::new();
    return match io::stdin().read_line(&mut value) {
        Ok(_) => Ok(value.trim().to_string()),
        Err(err) => Err(err),
    };
}

fn read_from_stdin_f64() -> Result<f64, String> {
    match read_string_from_stdin() {
        Ok(str) => {
            match str.parse() {
                Ok(num) => return Ok(num),
                Err(_) =>  return Err(String::from("Unable to parse integer")),
            };
        },
        Err(_) => return Err(String::from("Unable to parse integer")),
    };
}
