use std::io;

fn main() {
    let mut a: i8 = 1;
    println!("\nDuck cipher encrypter at your service :D\nenter the word quack to exit\n");
    while a == 1{

        let mut plaintext = String::new();
        println!("Enter plaintext string");
        io::stdin()
            .read_line(&mut plaintext)
            .expect("failed to read line");
        plaintext.retain(|c| !c.is_whitespace());
        


        if plaintext.to_lowercase() == "quack"{
            println!("\nended program successfully");
            a = 0;
        } else{
            println!("Your encrypted string: ");
            let str_vec: Vec<char> = plaintext.to_lowercase().chars().collect();
            for i in 0..plaintext.chars().count() {
                match str_vec[i] {
                    'a' => print!("quack "),
                    'b' => print!("qUack "),
                    'c'=> print!("quAck "),
                    'd' => print!("quaCk "),
                    'e' => print!("quacK "),
                    'f' => print!("Quack "),
                    'g' => print!("QUack "),
                    'h' => print!("QuAck "),
                    'i' => print!("QuaCk "),
                    'j' => print!("QuacK "),
                    'k' => print!("QUAck "),
                    'l' => print!("QUaCk "),
                    'm' => print!("QUacK "),
                    'n' => print!("QuACk "),
                    'o' => print!("QuAcK "),
                    'p' => print!("QuaCK "),
                    'q' => print!("QUACk "),
                    'r' => print!("QUACK "),
                    's' => print!("QuACK "),
                    't' => print!("QUaCK "),
                    'u' => print!("QUAcK "),
                    'v' => print!("QUACkk "),
                    'w' => print!("qUACK "),
                    'x' => print!("qUaCk "),
                    'y' => print!("quacck "),
                    'z' => print!("quaack "),
                    _ => print!("quacc "),
                }
            }
            println!(" ");
        }
    }
    println!(" ");

}
