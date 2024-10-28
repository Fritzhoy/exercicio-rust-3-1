use exercicio3_1::{calcular, maior, Estado, Operacao, Transporte};

//exercício 1
#[derive(Debug)]
struct Pessoa {
    nome: String,
    idade: u8,
    cidade: String,
}
// exercício 2
#[derive(Debug)]
struct Carro {
    marca: String,
    modelo: String,
    ano: i32,
}
// exercício 3
#[derive(Debug)]
struct Coordenada(f64, f64);
fn distancia(p1: Coordenada, p2: Coordenada) -> f64 {
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    (dx * dx + dy * dy).sqrt()
}
// exercício 4

struct Retangulo {
    largura: i32,
    altura: i32,
}
impl Retangulo {
    fn area(&self) -> i32 {
        self.altura * self.largura
    }
}
// exercício 5
enum Cor {
    Vermelho,
    Verde,
    Azul,
}
fn enum_simples() {
    let cor = Cor::Verde;

    match cor {
        Cor::Vermelho => println!("Vermelho"),
        Cor::Verde => println!("Verde"),
        Cor::Azul => print!("Azul"),
    }
}

fn main() {
    //1. Crianado e Usando Structs Simples
    let pessoa1 = Pessoa {
        nome: "George".to_string(),
        idade: 20,
        cidade: "Natal".to_string(),
    };
    let pessoa2 = Pessoa {
        nome: "Isabela".to_string(),
        idade: 30,
        cidade: "Brasilia".to_string(),
    };
    println!("Exercício 1");
    println!("Pessoa instanciada 1: {:?}", pessoa1);
    println!("Pessoa instanciada 2: {:?}", pessoa2);
    println!("=============");

    // 2. Atualizando Campos de uma Struct

    let carro1 = Carro {
        marca: "Audi".to_string(),
        modelo: "A4".to_string(),
        ano: 2019,
    };

    let mut carro2 = Carro { ..carro1 };
    carro2.ano = 2020;

    println!("Exercício 2");
    println!(": {:?}", carro2);
    println!("=============");

    //3. Struct com Tuples

    let gl1 = Coordenada(10.0, 20.0);
    let gl2 = Coordenada(5.1, 18.2);
    let result = distancia(gl1, gl2);

    println!("Exercício 3");
    println!("{:?}", result);
    println!("=============");

    // 4. Método em Struct

    let retangulo = Retangulo {
        altura: 50,
        largura: 25,
    };

    let resultado = retangulo.area();
    println!("Exercício 4");
    println!("Área do retangulo é: {}", resultado);
    println!("=============");

    // 5. Enumeração Simples
    println!("Exercício 5");
    enum_simples();
    println!("=============");

    //6. Enum com Parâmetros
    let multi = Operacao::Multiplicacao(10, 20);
    let resultado = calcular(multi);

    println!("Exercício 6");
    println!("Resultado: {}", resultado);
    println!("=============");

    //7. Enum `Option`
    println!("Exercício 7");

    let numeros = [5, 81, 107, 10, 12];
    match maior(&numeros) {
        Some(maior) => println!("Maior: {}", maior),
        None => println!("Lista vazia"),
    }
    println!("=============");

    //8. Funções Associadas em Enum
    let estado = Estado::Ligado;

    println!("Exercício 8");
    println!("Estado {}", estado.descricao());
    println!("=============");

    //9. Uso de `if let`
    println!("Exercício 9");
    if let Estado::Ligado = estado {
        println!("O dispositivo está ligado");
    } else {
        println!("O dispositivo está desligado");
    }
    println!("=============");

    //10. Match com Múltiplos Comandos

    println!("Exercício 10");

    let transporte = Transporte::Bicicleta;
    println!("{:?}", transporte.descricao());
    println!("=============");
}
