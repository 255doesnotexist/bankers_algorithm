// 导入必要的标准库和第三方库
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::process::exit;
use colored::*;

// 导入自定义模块
mod matrix;
use crate::matrix::Matrix;

mod test;

mod bankers_algorithm;
use bankers_algorithm::BankersAlgorithm;

fn main() {
    // 从文件中读取可用资源向量
    let available = read_vector("available.txt").expect("Failed to read available vector");
    let available_len = &available.len();

    // 从文件中读取最大需求矩阵和已分配矩阵
    let max: Matrix = read_matrix("max.txt").expect("Failed to read max matrix");
    let allocation = read_matrix("allocation.txt").expect("Failed to read allocation matrix");

    // 创建银行家算法实例
    let mut banker = BankersAlgorithm::new(available, max, allocation);
    
    // 打印系统初始状态
    println!("初始状态:");
    banker.print_state();
    
    // 执行初始安全性检查
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

    // 主循环：处理资源请求
    loop {
        // 读取进程 ID
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

        // 检查是否退出程序
        if process == -1 {
            break;
        }

        // 读取资源请求向量
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

        // 如果请求向量无效则重新开始
        if request.is_empty() {
            continue;
        }

        // 处理资源请求并打印新状态
        println!("\n进程 {} 请求资源: {:?}", process, request);
        banker.request_resources(process as usize, &request);
        banker.print_state();
    }
}

// 从文件中读取向量的辅助函数
fn read_vector(filename: &str) -> io::Result<Vec<i32>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // 读取第一行并将其转换为整数向量
    let line = reader.lines().next().unwrap()?;
    let vector: Vec<i32> = line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    Ok(vector)
}

// 从文件中读取矩阵的辅助函数
fn read_matrix(filename: &str) -> io::Result<Matrix> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // 逐行读取并转换为整数矩阵
    let mut data = Vec::new();
    for line in reader.lines() {
        let row: Vec<i32> = line?.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        data.push(row);
    }

    Ok(Matrix::from_vec(data))
}