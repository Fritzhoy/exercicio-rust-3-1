//6. Enum com Parâmetros
pub enum Operacao {
    Soma(i32, i32),
    Subtracao(i32, i32),
    Multiplicacao(i32, i32),
    Divisao(i32, i32),
    Potencia(i32),
}

pub fn calcular(operacao: Operacao) -> i32 {
    match operacao {
        Operacao::Soma(a, b) => a + b,
        Operacao::Subtracao(a, b) => a - b,
        Operacao::Multiplicacao(a, b) => a * b,
        Operacao::Divisao(a, b) => a / b,
        Operacao::Potencia(a) => a * a,
    }
}
//7. Enum `Option`

pub fn maior(numeros: &[i32]) -> Option<i32> {
    if numeros.is_empty() {
        None
    } else {
        Some(*numeros.iter().max().unwrap())
    }
}

// 8. Funções Associadas em Enums
pub enum Estado {
    Ligado,
    Desligado,
}
impl Estado {
    pub fn descricao(&self) -> &str {
        match self {
            Estado::Ligado => "Ligado",
            Estado::Desligado => "Desligado",
        }
    }
}

//10. Match com Múltiplos Comandos

pub enum Transporte {
    Carro,
    Bicicleta,
    Caminhada,
}

impl Transporte {
    pub fn descricao(&self) {
        match self {
            Transporte::Carro => {
                println!("Transporte: Carro");
                println!("Velocidade média: 50 km/h");
                println!("Impacto ambiental: Alto");
            }
            Transporte::Bicicleta => {
                println!("Transporte: Bicicleta");
                println!("Velocidade média: 18 km/h");
                println!("Impacto ambiental: Baixo");
            }
            Transporte::Caminhada => {
                println!("Transporte: Caminhada");
                println!("Velocidade média: 5 km/h");
                println!("Impacto ambiental: Nulo");
            }
        }
    }
}
