//Calculadora básica em rust!


//Funções

fn sum(a: f64, b: f64) -> f64 {
    a + b

}
fn sub(a: f64, b: f64) -> f64 {
    a - b
}
fn mul(a: f64, b: f64) -> f64 {
    a * b

}
fn divide(a: f64, b: f64) -> f64 {
    a / b

}

fn main() {
    println!("Olá, qual seria o primeiro número: ");
    let mut input1 = String::new();
    std::io::stdin().read_line(&mut input1).expect("Erro");
    let num1: f64 = input1.trim().parse().unwrap();
    println!("Segundo número: ");
    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("ERRO");
    let num2: f64  = input2.trim().parse().unwrap();
    println!("OPÇÕES: 1-Somar 2-subtrair 3-multiplicar 4-dividir");
    let mut input3 = String::new();
    std::io::stdin().read_line(&mut input3).expect("ERRO");
    let operador: i32 = input3.trim().parse().unwrap();
    let mut result: f64 = 0.0;
    match operador {
        1 => result = sum(num1, num2),
        2 => result = sub(num1, num2),
        3 => result = mul(num1, num2),
        4 => result = divide(num1, num2),
        _ => println!("ERROR, INVALID OPERATOR"),
        
    }
    println!("Result: {}", result);
}
