use rand::Rng;
use std::cmp::Ordering;
use std::{io, num};

fn main() {
    println!("Adivinhe o Número!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Digite seu palpite: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        print!("Você adivinhou: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito menor!"),
            Ordering::Greater => println!("Muito maior!"),
            Ordering::Equal => {
                println!("Você venceu!");
                break;
            }
        }
    }
}
