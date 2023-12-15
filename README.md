# DS210-HW6
This is my DS210 HW 6 Fall 2023

Full Credit 40/40

1. (14 points) The Fibonacci numbers, Fk
for k ∈ N, are defined a
0 if k = 0,
Fk = 1 if k = 1,
Fk−2 + Fk−1 otherwise
(a) Write a function fib that takes a parameter k of type u32 and returns an integer of type
u128 equal to Fk
. The computation of the function should exactly follow the definition
of Fibonacci numbers, i.e., to compute the value of the function for k ≥ 2, it should
recursively call fib twice.
Do not introduce any optimizations such as storing the numbers that have already
been computed. For large k, the solution will not fit into u128, but ignore this issue.
(b) Your function main should iterate from k = 0 to k = 50 and for each k, output the
following:
● k,
● Fk computed using fib,
● the time it took to compute Fk (see a suggestion how this can be done
below).
(c) Run your program twice: once via cargo run and once via cargo run --release. Since it
will take a lot of time for it to finish, interrupt it after a while. (Pressing Ctrl+C may work
for this purpose.)
Report: What are the computation times for different k for each of the options? How do
the times to compute Fibonacci numbers compare for large k between these two
options? Are they roughly the same or are they very different? If they are different, what
is the multiplicative difference?
Note: To find the amount if time it takes to perform some computation, you can
first place
use std::time::SystemTime;
at the beginning of your source file and then place the following around your computation.
let before = SystemTime::now();
// YOUR COMPUTATION HERE
let after = SystemTime::now();
let difference = after.duration_since(before);
let difference = difference.expect("Did the clock go back?");
println!("Time it took: {:?}", difference);
See https://doc.rust-lang.org/std/time/struct.SystemTime.html for more details.
2. (13 points)
(a) Create an array F of length 101 in which each entry is of type u128. Use the for loop
to make F[i] equal Fi, starting from i = 0 to i = 100. Then output all the computed
numbers Fi. Your solution should use only a constant number of arithmetic operations
to compute a given F[i] once the previous entries have been computed. This version
of your code should be the solution you return. Hint: Use previous entries in the array
to compute the current entry.
(b) Report: Now conduct the following experiment. Replace the array entry type with u8
and adjust any other types accordingly so your program still compiles. Try running the
modified code with both cargo run and cargo run --release. Are there any differences
in the behavior of the program? If so, what are they?
3. (13 points) Write a program that reads a non-negative integer—let us denote it k—and
computes Your solution should use the for loop to compute it directly as defined (even
𝑖=1
𝑘
∑ 𝑖
2
though there is a smarter way to do this). Assume that the input k can be represented, using
u32. Under this assumption, make sure that the output is computed correctly, i.e., that
throughout the computation you do not use an integer type of range too small to represent your
intermediate and final numbers.
Report: Explain why the situation described above is not happening, i.e., why the range of
integers you use is sufficiently large. This kind of problem is known as integer overflow, i.e., you
want to explain why integer overflow is not a problem in your code.
Note: To read a line and convert it to an integer, you can do the following. First, put this
statement at the beginning of your file:
use std::io;
Then use the following when you want to read an integer:
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Failed to read line");
let input = input.trim();
let number: u32 = input.parse().expect("Not a good number!");
4. (Optional, no credit)
Report: How much time did you spend on this homework? The answer will have no impact on
the credit you receive, but it may help us adjust the difficulty of future homework assignments
