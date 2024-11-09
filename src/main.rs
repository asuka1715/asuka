// main.rs

mod calculator;
mod standard_sort;
mod quick_sort;

use std::io;

fn main() {
    println!("选择功能：\n1. 计算器\n2. 排序");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("无法读取输入");
    let choice: u32 = choice.trim().parse().expect("请输入有效的数字");

    match choice {
        1 => calculator::calculate(),
        2 => {
            let numbers = read_numbers();
            if numbers.is_empty() {
                println!("未输入有效数字。");
                return;
            }

            println!("选择排序方式：\n1. 标准库排序\n2. 快速排序");
            let mut sort_choice = String::new();
            io::stdin().read_line(&mut sort_choice).expect("无法读取输入");
            let sort_choice: u32 = sort_choice.trim().parse().expect("请输入有效的数字");

            let mut numbers = numbers; // 可变以便排序
            match sort_choice {
                1 => {
                    standard_sort::sort(&mut numbers);
                    println!("使用标准库排序结果: {:?}", numbers);
                }
                2 => {
                    quick_sort::sort(&mut numbers);
                    println!("使用快速排序结果: {:?}", numbers);
                }
                _ => println!("无效选择"),
            }
        }
        _ => println!("无效选择"),
    }
}

// 读取用户输入的数字，并返回一个 Vec<i32>
fn read_numbers() -> Vec<i32> {
    println!("请输入要排序的数字（用空格分隔）：");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("无法读取输入");
    input
        .trim()
        .split_whitespace()
        .filter_map(|num| num.parse().ok())
        .collect()
}
