use std::{io};
use num_bigint::BigUint;
use num_traits::{Zero, One};

fn main() {

    println!("Starting program.");

    let x = true;

    //Main loop starts here. It gets an input from the user and checks if it is an integer.
    while x == true {

        println!("Enter your number here:");

        let mut input: String = String::new();
        io::stdin().read_line(&mut input);

        let input_trim: &str = input.trim();

        if input_trim == "exit" || input_trim == "c" {
            break;
        }

        let val: i32 = input.trim().parse().unwrap_or(-2);

        if val == -2 {
            println!("Not a valid number. Try again.");

            continue;
        }

        println!("Your number is: {}", val);

        fib(val);

        println!(" ");

        println!("Try another number or enter 'exit' and press enter to end the program.");
    
    }

}


fn fib(n: i32) {

    println!("Calculating your Fibonacci number:");

    if n >= 10001 {
        println!("Number is too large! Max size is 10000. :'( ");
        return;
    }


    if n <= 0 {
        println!("Invalid Fibonacci Number.");
        return;
    }

    if n == 1 {
        println!("{}", n);
        return;
    }

    let mut previous : BigUint = One::one();
    let mut current: BigUint = One::one();

    println!("{}", previous);
    println!("{}", current);

    for _ in 2..n {
        let next: BigUint = previous + &current;

        println!("{}", next);
        println!(" ");

        previous = current;
        current = next;
    }

    println!(" ");
    println!(" ");

    println!("The {}-th number in the Fibonacci sequence is: ", n);

    println!("{}", current);
    println!(" ");

    return;

}
