use std::io; 
use std::cmp::Ordering;
use rand::Rng;



fn main() {
    println!("Adivinhe o Número!");

    // Gera um número aleatóri de 1 até 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop { // Loop infinito

        println!("Por favor entre com seu palpite.");

        let mut guess = String::new(); // Cria uma string

        // Lê da entrada padrão
        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a linha.");

        // Tenta converter para um inteiro
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Seu palpite: {guess}");

        // Compara a entrada com o número aleatório
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Seu palpite é Menor!"),
            Ordering::Greater => println!("Seu palpite é Maior"),
            Ordering::Equal => {
                println!("Você acertou!!!");
                break; // Se acertar sai do loop
            }
        }
    } 

}
