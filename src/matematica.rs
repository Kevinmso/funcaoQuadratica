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
                raizes.push((-self.b + delta.sqrt()) / (2.0 * self.a));
                raizes.push((-self.b - delta.sqrt()) / (2.0 * self.a));
            }

            _ => ()
        }

        if raizes.len() == 2 && raizes[0] == raizes[1]{
            raizes.pop();      
        }

        raizes
    }
    
    // calcula o vértice da função
    pub fn calcular_vertice(&self) -> (f64, f64) {
        let x = -self.b / (2.0 * self.a);
        let y = -self.calcular_delta() / (4.0 * self.a);
        (x, y)
    }
}