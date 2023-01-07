fn main() {
    let notas: [f32; 4] = [10f32, 8f32, 9.5, 6.0];
    let inteiro: usize = 0;

    println!("{}", notas[inteiro]);

    for nota in notas {
        println!("Nota = {}", nota);
    }

    let notasSecond: [f32; 4] = [6.5; 4];
    for indice in 0..notasSecond.len() {
        println!("A nota {} é = {}", indice + 1, notasSecond[indice]);
    }

    matriz();
    println!("É fim de semana? {}", eh_fim_de_semana(DiaDaSemana::Quarta));

    let dia: DiaDaSemana = DiaDaSemana::Sexta;
    cores();
    conteudo_opcional();
    vectors();
    conta_corrente();
}

enum DiaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
}

fn matriz() {
    let matriz: [[f32; 3]; 2] = [[0.0, 1.2, 0.1], [1.3, 0.3, 1.4]];
    for linha in matriz {
        for item in linha {
            println!("Item = {}", item)
        }
    }
}

fn eh_fim_de_semana(dia_da_semana: DiaDaSemana) -> bool {
    match dia_da_semana {
        DiaDaSemana::Domingo | DiaDaSemana::Sabado => true,
        _ => false,
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CymkColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

fn cores() {
    let cor = Color::CymkColor {
        cyan: 100,
        magenta: 50,
        yellow: 70,
        black: 200,
    };

    println!(
        "Cor = {}",
        match cor {
            Color::Red => "vermelho",
            Color::Green => "verde",
            Color::Blue => "blue",
            Color::RgbColor(0, 0, 0)
            | Color::CymkColor {
                cyan: _,
                magenta: _,
                yellow: _,
                black: 255,
            } => "preta",
            Color::RgbColor(_, _, _) => "RGB desconhecido",
            Color::CymkColor {
                cyan: _,
                magenta: _,
                yellow: _,
                black: _,
            } => "desconhecido",
        }
    )
}

fn conteudo_opcional() {
    let conteudo_arquivo = ler_arquivo(String::from(""));
    match &conteudo_arquivo {
        Some(valor) => println!("{}", valor),
        None => println!("Arquivo nao existe"),
    };

    println!("{:?}", &conteudo_arquivo);

    if let Some(valor) = conteudo_arquivo {
        println!("Agora, tenho certeza de ha o valor {}", valor);
    }
}

fn ler_arquivo(caminho_arquivo: String) -> Option<String> {
    Some(String::from("Algum conteudo"))
}

fn vectors() {
    let mut notas: Vec<f32> = Vec::new();
    let mut notasTwo: Vec<f32> = vec![10.0, 8.0, 6.5];
    let mut notasWithCapacity: Vec<f32> = Vec::with_capacity(4);
    println!("Capacidade = {}", notasWithCapacity.capacity());
    println!("Capacidade = {}", notas.capacity());
    println!("{:?}", notas);
    notas.push(10.0);
    notas.push(8.8);
    notas.push(6.5);
    println!("Capacidade = {}", notas.capacity());
    println!("{:?}", notas);

    println!("Nota 1 = {}", notasTwo[0]);

    println!(
        "Nota 6 = {}",
        match notas.get(3) {
            Some(n) => *n,
            None => 0.0,
        }
    );
    // dara problemas de bowrrow pois ele vai pegar o valor e levar para o contexto do for
    for nota in &notas {
        println!("Nota = {}", nota);
    }

    println!("{:?}", notas);

    if let Some(nota) = notas.pop() {
        println!("Ultimo valor = {}", nota);
        println!("{:?}", notas);
    }

    while let Some(nota) = notas.pop() {
        println!("Ultimo valor = {}", nota);
        println!("{:?}", notas);
    }
}

struct Titular {
    nome: String,
    sobrenome: String,
}
struct Conta {
    tittular: Titular,
    saldo: f64,
}
impl Conta {
    fn sacar(&mut self, valor: f64) {
        self.saldo -= valor;
    }
}
fn conta_corrente() {
    let titular: Titular = Titular {
        nome: String::from("Anderson"),
        sobrenome: String::from("Fernandes"),
    };
    let mut conta: Conta = Conta {
        tittular: titular,
        saldo: 100.0,
    };
    conta.sacar(50.0);
    println!(
        "Dados da conta: Titular = {} {}, Saldo = {}",
        conta.tittular.nome, conta.tittular.sobrenome, conta.saldo
    );
}
