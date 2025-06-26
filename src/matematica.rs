use std::ops::Range;

// criando a estrutura onde se enquadrarão equações quadráticas
pub struct Quadratica{
    a: f64,
    b: f64,
    c: f64,
}

impl Quadratica{
    pub fn new(a: f64, b: f64, c: f64) -> Quadratica{
        Quadratica{a, b, c}
    }

    // recebe valor x e calcula f(x)
    pub fn calcular_valor(&self, x: f64) -> f64{
        self.a * x.powi(2) + self.b * x + self.c
    }

    // calcula o delta da função
    pub fn calcular_delta(&self) -> f64{
        self.b.powi(2) - 4.0 * self.a * self.c
    }

    // calculas as raízes da função
    pub fn calcular_raizes(&self) -> Vec<f64>{
        let mut raizes: Vec<f64> = Vec::new();
        let delta: f64 = self.calcular_delta();

        match delta{
            d if d >= 0.0 => {
                let raiz_delta = delta.sqrt();
                raizes.push((-self.b + raiz_delta) / (2.0 * self.a));
                raizes.push((-self.b - raiz_delta) / (2.0 * self.a));
            }

            _ => ()
        }

        if raizes.len() == 2 && raizes[0] == raizes[1]{
            raizes.pop();
        }

        raizes
    }
    
    // calcula ponto onde a parábola intercepta o eixo y
    pub fn calcular_intercepto_y(&self) -> (f64, f64){
        (0.0, self.calcular_valor(0.0))
    }

    // calcula o vértice da função
    pub fn calcular_vertice(&self) -> (f64, f64) {
        let x = -self.b / (2.0 * self.a);
        let y = -self.calcular_delta() / (4.0 * self.a);
        (x, y)
    }

    // método que calcula f(x) para todos os valores de x necessários
    pub fn gerar_pontos(&self, intervalo: Range<f64>, passos: usize) -> Vec<(f64, f64)>{
        let mut pontos: Vec<(f64, f64)> = Vec::new();
        let passo = (intervalo.end - intervalo.start) / passos as f64;

        for i in 0..=passos{
            let x = intervalo.start + i as f64 * passo;
            pontos.push((x, self.calcular_valor(x)));
        }

        pontos
    }
}