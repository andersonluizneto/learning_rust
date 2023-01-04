fn main() {

    let mut x = 5; // Variável mutável
    
    println!("O valor de x é: {x}");
    x = 6;
    println!("O valor de x é: {x}");

    const PONTOS_MAXIMOS: u32 = 100_000;
    println!("O valor da constante PONTOS_MAXIMOS é: {PONTOS_MAXIMOS}");

    // Shadowing (sombreamento)
    let y = 6;
    let y = y + 1;
    let y = y * 3;
    println!("O valor de y é: {y}");
}
