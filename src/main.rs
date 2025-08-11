// use calculator_washinribeiro::calc1::{add, sub};
// use calculator_washinribeiro::calc2::{multiply, rate};
// use calculator_washinribeiro::calc3::{pot, log};
use calculator_washinribeiro::calc1::{add};

fn main() {
    println!("\n---- Testando a Biblioteca Calculatora ----");

    let c = add(10, 30);
    println!("Soma de 10 e 30: {:?}", c);

    println!("---- Fim dos testes manuais ----")
}