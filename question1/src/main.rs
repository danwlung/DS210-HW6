//Question 1

//Part A
use std::time::SystemTime;

//Fibonacci function


fn fib(n: u32) -> u128 {
    if n <= 0{
        return 0;
    }
    else if n == 1{
        return 1;
    }
    else{
        return fib(n-1) + fib(n-2)
    }
}

//Part B

//Main function
fn main() {
    for i in 1..51{
        let before = SystemTime::now();
        println!("The Fibonacci number for {} is {}", i, fib(i));
        let after = SystemTime::now();
        let difference = after.duration_since(before);
        let difference = difference.expect("Did the clock go back?"); 
        println!("Time it took: {:?}", difference);
    }
}


        