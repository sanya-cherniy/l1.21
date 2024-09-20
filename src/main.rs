use rust_decimal::prelude::*; // используем библиотеку реализующую тип Decimal который использует 128 бит для хранения
use rust_decimal::Decimal;

use std::io;
fn main() {
    println!("Input value 1:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let value_1 = Decimal::from_str(&input.trim_end()).unwrap(); // записываем значение 1
    input.clear();
    println!("Input value 2:");
    io::stdin().read_line(&mut input).unwrap();
    let value_2 = Decimal::from_str(&input.trim_end()).unwrap(); // записываем значение 2
    println!("Select operation: \n 1. Addition\n 2. Subtraction\n 3. Multiplication\n 4. Division");

    let mut num = String::new();

    io::stdin().read_line(&mut num).unwrap();

    let num: u8 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Incorrect operation number");
            return;
        }
    };
    let res;
    match num {
        // в зависимости от ввода пользователя выполняем операции
        1 => res = value_1 + value_2,
        2 => res = value_1 - value_2,
        3 => res = value_1 * value_2,
        4 => res = value_1 / value_2,
        _ => {
            println!("Incorrect operation number");
            return;
        }
    };
    println!("Result: {}", res);
}
