use crate::matematica::Quadratica;

mod matematica;

fn main() {
    // testes na main
    let equacao = Quadratica::new(1.0, 2.0, 5.0);
    let x = 1.0;
    
    println!("{}", equacao.calcular_valor(x));
    println!("{:?}", equacao.calcular_raizes());
    println!("{:?}", equacao.calcular_vertice());
    println!("{:?}", equacao.gerar_pontos(0.0..5.0, 500));
}