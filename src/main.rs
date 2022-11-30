use std::io;

fn factorial(n: u32, index: u32) -> u32 {
    if index == 1 {
        return n;
    }

    return factorial(n * index, index - 1);
}

fn loop_factorial(n: u32) -> u32 {
    let mut result = n;
    let mut flag = n - 1;
    loop {
        if flag == 1 {
            break;
        }
        result = result * flag;
        flag = flag - 1;
    }

    return result;
}

fn main() {
    println!("Hello, world! This is a test to calculate the factorial (n!) of a number.");
    println!("If you want to exit at any time, just type in 'exit'");

    loop {    
        println!("Let's find the factorial for any number, please input a number here:");
    
        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read number");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                if number.trim() == "exit" {
                    break;
                }
                continue;
            }
        };

        let result: u32 = loop_factorial(number);
        println!("The result is: {result}");
    }
}
