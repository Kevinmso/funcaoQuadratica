use crate::matematica::Quadratica;
use std::io::{self, Write};

mod matematica;
mod ui;

fn ler_numero(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse::<f64>() {
            Ok(numero) => return numero,
            Err(_) => println!("Por favor, digite um número válido!"),
        }
    }
}

fn main() {
    println!("Bem-vindo ao Gerador de Função Quadrática!");
    println!("Digite os coeficientes da função f(x) = ax² + bx + c");
    
    let a = ler_numero("Digite o valor de a: ");
    let b = ler_numero("Digite o valor de b: ");
    let c = ler_numero("Digite o valor de c: ");
    
    println!("\nFunção digitada: f(x) = {}x² + {}x + {}", a, b, c);
    println!("Iniciando interface gráfica...\n");

    // Criar e iniciar a interface gráfica com os valores fornecidos
    ui::criar_app_com_valores(a, b, c).ok();
}