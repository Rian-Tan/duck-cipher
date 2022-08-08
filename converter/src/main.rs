use std::io;

fn main() {
    let mut plaintext = String::new();
    println!("Enter plaintext string");
    io::stdin()
        .read_line(&mut plaintext)
        .expect("failed to read line");
    
    println!("Your encrypted string:");
    for _i in 1..plaintext.chars().count(){
        print!("quack ");
    }
    println!(" ");

}
