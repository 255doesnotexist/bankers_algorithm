use crate::matrix::Matrix;
use colored::*;

pub struct BankersAlgorithm {
    available: Vec<i32>,
    max: Matrix,
    allocation: Matrix,
    need: Matrix,
    num_processes: usize,
    num_resources: usize,
}

impl BankersAlgorithm {
    pub fn new(
        available: Vec<i32>,
        max: Matrix,
        allocation: Matrix,
    ) -> Self {
        let num_processes = max.rows;
        let num_resources = max.cols;
        let mut need = Matrix::new(num_processes, num_resources);

        // 计算初始的需求矩阵
        for i in 0..num_processes {
            for j in 0..num_resources {
                need.data[i][j] = max.data[i][j] - allocation.data[i][j];
            }
        }

        BankersAlgorithm {
            available,
            max,
            allocation,
            need,
            num_processes,
            num_resources,
        }
    }

    /// 检查系统是否处于安全状态
    /// 
    /// # 返回值
    /// * `(bool, Vec<usize>)` - (是否安全，安全序列)
    pub fn is_safe(&self) -> (bool, Vec<usize>) {
        let mut work = self.available.clone();
        let mut finish = vec![false; self.num_processes];
        let mut safe_sequence = Vec::new();

        // 安全性检查的主循环
        loop {
            let mut found = false;
            // 查找一个可以满足的进程
            for i in 0..self.num_processes {
                if !finish[i] && self.can_allocate(&work, i) {
                    // 模拟分配过程
                    for j in 0..self.num_resources {
                        work[j] += self.allocation.data[i][j];
                    }
                    finish[i] = true;
                    safe_sequence.push(i);
                    found = true;
                    
                    println!("{}进程 {} 已分配。工作向量: {:?}",
                        "→ ".green(), i, work);
                }
            }
            
            if !found {
                break;
            }
        }

        let is_safe = finish.iter().all(|&x| x);
        (is_safe, safe_sequence)
    }


    fn can_allocate(&self, work: &Vec<i32>, process: usize) -> bool {
        (0..self.num_resources).all(|j| self.need.data[process][j] <= work[j])
    }

    pub fn request_resources(&mut self, process: usize, request: &Vec<i32>) -> bool {
        // 验证请求是否合法
        if !self.is_request_valid(process, request) {
            println!("{} Request exceeds maximum claim", "ERROR:".red());
            return false;
        }

        // 验证资源是否足够
        if !self.has_sufficient_resources(request) {
            println!("{} Insufficient resources", "ERROR:".red());
            return false;
        }

        // 尝试分配资源
        self.try_allocation(process, request);

        // 检查安全性
        let (is_safe, sequence) = self.is_safe();
        
        if is_safe {
            println!("{} Resource allocation successful", "SUCCESS:".green());
            println!("Safe sequence: {:?}", sequence);
            true
        } else {
            // 回滚分配
            self.rollback_allocation(process, request);
            println!("{} Allocation would lead to unsafe state", "ERROR:".red());
            false
        }
    }

    fn is_request_valid(&self, process: usize, request: &Vec<i32>) -> bool {
        (0..self.num_resources).all(|j| request[j] <= self.need.data[process][j])
    }

    fn has_sufficient_resources(&self, request: &Vec<i32>) -> bool {
        (0..self.num_resources).all(|j| request[j] <= self.available[j])
    }

    fn try_allocation(&mut self, process: usize, request: &Vec<i32>) {
        for j in 0..self.num_resources {
            self.available[j] -= request[j];
            self.allocation.data[process][j] += request[j];
            self.need.data[process][j] -= request[j];
        }
    }

    fn rollback_allocation(&mut self, process: usize, request: &Vec<i32>) {
        for j in 0..self.num_resources {
            self.available[j] += request[j];
            self.allocation.data[process][j] -= request[j];
            self.need.data[process][j] += request[j];
        }
    }

    pub fn print_state(&self) {
        println!("\n{}", "Current System State:".yellow());
        println!("Available: {:?}", self.available);
        println!("\nMaximum:");
        print!("{}", self.max);
        println!("\nAllocation:");
        print!("{}", self.allocation);
        println!("\nNeed:");
        print!("{}", self.need);
        println!();
    }
}