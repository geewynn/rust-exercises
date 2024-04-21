
fn main() {    
    let generated_num = rand::thread_rng().gen_range(1..101);
    println!("I'm thinking of a number between 1 and 100...");
    println!("Guess the number:");
    loop {
            let mut num = String::new();
            io::stdin().read_line(&mut num).expect("Failed to read input line.");
            let num: u32 = num.trim().parse().expect("failed to parse num");
        
            if num == generated_num {
                println!("guessed correctly {}", generated_num);
                break;
            } else if num > generated_num {
                println!("{} is greater  than gen {}", num, generated_num);
            } else {
                println!("{} is less than gen {}", num, generated_num)
            }
        }
    



}