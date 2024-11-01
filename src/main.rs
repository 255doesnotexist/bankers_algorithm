use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::exit;
use colored::*;

mod matrix;
use crate::matrix::Matrix;

mod test;

mod bankers_algorithm;
use bankers_algorithm::BankersAlgorithm;

fn main() {
    let available = read_vector("available.txt").expect("Failed to read available vector");
    let available_len = &available.len();

    let max: Matrix = read_matrix("max.txt").expect("Failed to read max matrix");
    let allocation = read_matrix("allocation.txt").expect("Failed to read allocation matrix");

    let mut banker = BankersAlgorithm::new(available, max, allocation);
    
    println!("初始状态:");
    banker.print_state();
    
    println!("\n运行系统初始安全性检查...");
    let (is_safe, sequence) = banker.is_safe();
    if is_safe {
        println!("{} 系统处于安全状态", "SUCCESS:".green());
        println!("安全序列: {:?}", sequence);
    } else {
        println!("{} 系统处于不安全状态", "ERROR:".red());
        println!("{}", "无法继续执行操作，即将退出程序。请尝试修改 max、allocation 矩阵和 available 向量保证系统初始状态安全。".red());
        exit(-1);
    }
    loop {
        // 从 stdin 读取请求资源的进程 ID
        print!("\n请输入请求资源的进程 ID (-1 退出): ");
        io::stdout().flush().unwrap();
        let mut process_input = String::new();
        io::stdin().read_line(&mut process_input).expect("Failed to read line");
        let process: i32 = match process_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入一个有效的数字");
                continue;
            }
        };

        if process == -1 {
            break;
        }

        // 从 stdin 读取 request 向量
        let mut request = Vec::new();
        for i in 0..*available_len {
            print!("请输入请求的第 {} 个资源数量: ", i + 1);
            io::stdout().flush().unwrap();
            let mut request_input = String::new();
            io::stdin().read_line(&mut request_input).expect("Failed to read line");
            let resource: i32 = match request_input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("请输入一个有效的数字");
                    request.clear();
                    break;
                }
            };
            request.push(resource);
        }

        if request.is_empty() {
            continue;
        }

        println!("\n进程 {} 请求资源: {:?}", process, request);
        banker.request_resources(process as usize, &request);

        banker.print_state();
    }
}

fn read_vector(filename: &str) -> io::Result<Vec<i32>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let line = reader.lines().next().unwrap()?;
    let vector: Vec<i32> = line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    Ok(vector)
}

fn read_matrix(filename: &str) -> io::Result<Matrix> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut data = Vec::new();
    for line in reader.lines() {
        let row: Vec<i32> = line?.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        data.push(row);
    }

    Ok(Matrix::from_vec(data))
}