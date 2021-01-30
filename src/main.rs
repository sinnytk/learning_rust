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
fn fahr_to_cel(tmp_in_fahr : f64) -> f64{
    (tmp_in_fahr - 32.0) * 5.0/9.0
}
fn cel_to_fahr(tmp_in_cel : f64) -> f64{
    (tmp_in_cel * 9.0/5.0) + 32.0
}
fn main() {
    println!("What function do you want to run?");
    println!("1: Print nth Fibonacci term");
    println!("2: Convert celsius to fahrenheit");
    println!("3: Convert fahrenheit to celsius");
    println!("\n");
    println!("Enter valid function number:");

    let mut function_input = String::new();
    io::stdin()
        .read_line(&mut function_input)
        .expect("Failed to read a line");

    let function_input:i32= function_input.trim()
                 .parse()
                 .expect("Please make sure input is a number");

    match function_input{
        1 => {
            // Fibonacci
            // Case block takes as input an unsigned number `n`
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

        2 | 3 => {
            // Celsius to Fahrenheit or Fahrenheit to Celsius
            // This case block takes as input a floating point value `temp`
            println!("Enter temperature: ");

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read a line");

            let temp:f64= input.trim()
                        .parse()
                        .expect("Please make sure input is a number(can be float)");

            // Case block will call cel_to_fahr if 2
            if function_input == 2 {
                println!("{}℃ is {}℉",temp,cel_to_fahr(temp));
            }
            else{
                println!("{}℉ is {}℃",temp,fahr_to_cel(temp));

            }
        }
        _ => {
            println!("Invalid function number!, try again..")
        }
    }

}
