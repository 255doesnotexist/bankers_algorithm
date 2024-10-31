use colored::*;

mod matrix;
use crate::matrix::Matrix;

mod test;

mod bankers_algorithm;
use bankers_algorithm::BankersAlgorithm;

fn main() {
    // 示例用例1
    let available = vec![3, 3, 2];
    let max = Matrix::from_vec(vec![
        vec![7, 5, 3],
        vec![3, 2, 2],
        vec![9, 0, 2],
        vec![2, 2, 2],
        vec![4, 3, 3],
    ]);
    let allocation = Matrix::from_vec(vec![
        vec![0, 1, 0],
        vec![2, 0, 0],
        vec![3, 0, 2],
        vec![2, 1, 1],
        vec![0, 0, 2],
    ]);

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