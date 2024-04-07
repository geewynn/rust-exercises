// Define a function named 'fib' that calculates the nth Fibonacci number
fn fib(n: u32) -> u32 { 
    if n <= 2 {
        // Base case: If n is 0, 1, or 2, the Fibonacci number is 1
        1
    } else {
        // Recursive case: The Fibonacci number is the sum of the two previous Fibonacci numbers
        fib(n - 1) + fib(n-2) 
    }
}

fn main() {
    let n = 20; // Set the value of n to calculate the 20th Fibonacci number
    println!("fib({n}) = {}", fib(n)); // Calculate and print the result
}