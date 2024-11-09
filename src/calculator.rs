// calculator.rs

use std::io;

pub fn calculate() {
    println!("输入第一个数字:");
    let num1 = read_number();

    println!("输入第二个数字:");
    let num2 = read_number();

    println!("选择操作 (+, -, *, /):");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator).expect("无法读取输入");
    let operator = operator.trim();

    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => {
            println!("无效操作符");
            return;
        }
    };

    println!("结果是: {}", result);
}

fn read_number() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("无法读取输入");
    input.trim().parse().expect("请输入有效的数字")
}
