fn collatz_length(mut n: i32) -> u32 {
    let mut steps = 0;
    while n > 1 {
        if n % 2 == 0 {
            n/=2;
        } else {
            n = 3 * n + 1;
        }
        steps += 1;
    }
    steps
}


#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

fn main() {
    println!("Length: {}", collatz_length(11));
}