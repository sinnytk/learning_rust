use std::io;
fn nth_fib(n:u32) -> u32{
    match n {
        1 => 0,
        2 => 1,
        _ => {
            let mut counter = 2;
            let mut a = 0;
            let mut b = 1;
            while counter < n {
                let new = a+b;
                a=b;
                b=new;
                counter+=1;
            }
            b
        }
    }
}

fn main() {
    // Fibonacci
    // Main function takes as input an unsigned number `n`
    // Calls `nth_fib_num` function with `n` as argument
    // Function `nth_fib_num` as the name suggests, returns nth Fibonacci number

    println!("Enter any positive number greater than 0: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read a line");

    let n:u32= input.trim()
                 .parse()
                 .expect("Please make sure input is a number");

    let nth_fib_num = nth_fib(n);
    println!("{}th fibonacci number is: {}",n,nth_fib_num);
}
