fn lesson_one() {
    let mut name: &str = "Fabio";
    name = "Henrique";
    println!("Hello, {}!", name);
}

fn lesson_two() {
    let x: u64 = 23;
    let f: f64 = 6.7;
    let b: bool = false;
    println!("Hello, {}!", x);
}

fn lesson_three() {
    let number1 = 24;
    let number2 = 42;

    if number1 > number2 {
        print!("{} > {}", number1, number2)
    } else {
        print!("{} <= {}", number1, number2)
    }
}

fn convert_to_int(data_input: &String) -> i32 {
    let x = data_input.trim().parse::<i32>().unwrap();
    x
}
const PI: f32 = 3.14;
static mut GLOBAL: u8 = 1;

fn scopo() {
    let variavel = 300;
    println!("PI = {}", PI);
    //& <- traz o endereço na memoria
    println!(
        "variavel = {}, tamanho = {} bytes",
        variavel,
        std::mem::size_of_val(&variavel)
    );
    unsafe { println!("vairavel_global {}", GLOBAL) }
    let decimal: f32 = 2.5;
    println!("decimal = {}", decimal);

    let booleana: bool = true;
    println!(
        "Boolean = {}, Tamanho booleana = {}",
        booleana,
        std::mem::size_of_val(&booleana)
    );

    let letra: char = 'C';
    println!("tamanho do char = {}", std::mem::size_of_val(&letra));

    println!("decimal {}", soma(1, 2))
}

fn condicionais() {
    let idade: u8 = 17;
    let responsavel_autorizou: bool = true;

    if idade > 18 {
        println!("pode entrar na balada");
    } else if idade > 16 && responsavel_autorizou {
        println!("Pode entrar com assinatura do responsavel");
    } else {
        println!("Não pode entrar na balada");
    }

    let condicao: &str = if idade >= 18 { "maior" } else { "menor" };
    println!("É {} de idade", condicao);

    let linguagem = "PHP";
    let proposito = match linguagem {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Desconhecido",
    };

    println!("linguagem {} e proposito {}", linguagem, proposito);
}
fn main() {
    scopo();
    sombra();
    println!("Soma = {}", soma(2, 2));
    condicionais();
    repeticoes();
    ownership();
    pattern_matching();
    erros();
}

fn soma(a: i32, b: i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    a + b
}

fn sombra() {
    let a = 123;

    {
        let b = 456;
        println!("dentro, b = {}", b);

        let a = 777;
        println!("dentro, a = {}", a);
    }

    println!("fora, a = {}", a);
}

fn repeticoes() {
    let multiplicador: u8 = 5;

    let mut contador: u8 = 0;
    while contador < 10 {
        contador += 1;

        if contador == 5 {
            continue;
        }

        println!(
            "{} x {} = {}",
            multiplicador,
            contador,
            multiplicador * contador
        )
    }

    contador = 0;
    loop {
        contador += 1;
        println!(
            "{} x {} = {}",
            multiplicador,
            contador,
            multiplicador * contador
        );

        if contador == 10 {
            break;
        }
    }

    for i in 1..=10 {
        println!("{} x {} = {}", multiplicador, i, multiplicador * i);
    }
}

fn ownership() {
    let uma_string = String::from("Vinicius");
    let mut outra_string = rouba_devolve_valor(uma_string);

    println!("{}", outra_string);

    empresta_string(&outra_string);
    empresta_string_mutable(&mut outra_string);
}
fn rouba(string: String) {
    // aqui o contexto muda e o objeto dinamico esta dentro deste contexto então quando este contexto morre ele apaga
    // o objeto que esta na memoria heap
    println!("{}", string);
}

fn rouba_devolve_valor(string: String) -> String {
    // aqui o contexto muda e o objeto dinamico esta dentro deste contexto então quando este contexto morre ele apaga
    // o objeto que esta na memoria heap
    println!("{}", string);
    //mas devolve outra string
    string
}

fn empresta_string(string: &String) {
    // aqui o contexto muda e porém pegamos apenas a referencia do objeto
    // o objeto que esta na memoria heap
    println!("{}", string);
}

fn empresta_string_mutable(string: &mut String) {
    string.push_str(" Dias");
    // aqui o contexto muda e porém pegamos apenas a referencia do objeto
    // o objeto que esta na memoria heap
    println!("{}", string);
}

fn pattern_matching() {
    for x in 1..=20 {
        println!(
            "{}: {}",
            x,
            match x {
                1 => "Pouco",
                2 | 3 => "Pouquinho",
                4..=10 => "Um bocado",
                _ if x % 2 == 0 => "Uma boa quantidade",
                _ => "Muito",
            }
        )
    }
}

fn erros() {
    match resultado() {
        Ok(s) => println!("string de sucesso = {}", s),
        Err(numero) => println!("código de erro = {}", numero),
    }
}

fn resultado() -> Result<String, u8> {
    // Ok(String::from("Tudo deu certo"))
    Err(42)
}
