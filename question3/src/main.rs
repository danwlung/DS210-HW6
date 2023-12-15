//Question 3

use std::io;

fn main() {
    //For reading integer
    println!{"Enter a non-negative integer:"};
    let mut k = String::new();
    io::stdin().read_line(&mut k).expect("Failed to read line"); 
    let k: u32 = k.trim().parse().expect("Not a good number!");

    //Doing the question
    let mut sum : u32 = 0;
    for i in 0..k+1{
        sum = sum + (i*i);
    }

    println!("input: {}", k);
    println!("sum of i^2: {}", sum);
}