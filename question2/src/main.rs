//Question 2

fn main() {
    let mut f_array: [u128; 101] = [0; 101];
    for i in 0..f_array.len() {
        if i == 0 {
            f_array[i] = 0;
        }
        else if i == 1 {
            f_array[i] = 1;
        }
        else {
            f_array[i] = f_array[i-1] + f_array[i-2];
        }
    }

    println!("The Fibonacci number of each index in the array is: {:?}", f_array);

}