use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use colored::*;

mod matrix;
use crate::matrix::Matrix;

mod test;

mod bankers_algorithm;
use bankers_algorithm::BankersAlgorithm;

fn main() {
    let available = read_vector("available.txt").expect("Failed to read available vector");
    let max = read_matrix("max.txt").expect("Failed to read max matrix");
    let allocation = read_matrix("allocation.txt").expect("Failed to read allocation matrix");

    let mut banker = BankersAlgorithm::new(available, max, allocation);
    
    println!("初始状态:");
    banker.print_state();
    
    println!("\n运行安全性检查...");
    let (is_safe, sequence) = banker.is_safe();
    if is_safe {
        println!("{} 系统处于安全状态", "SUCCESS:".green());
        println!("安全序列: {:?}", sequence);
    } else {
        println!("{} 系统处于不安全状态", "ERROR:".red());
    }
    // 处理资源请求
    let process = 1;
    let request = vec![1, 0, 2];
    println!("\n进程 {} 请求资源: {:?}", process, request);
    banker.request_resources(process, &request);
    
    banker.print_state();
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