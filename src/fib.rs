use num_bigint::BigUint;
use num_traits::One;


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