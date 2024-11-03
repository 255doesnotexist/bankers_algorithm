use crate::matrix::Matrix;
use prettytable::{Table, Row, Cell, format};
use colored::*;

/// 银行家算法的主要结构体
/// 包含所有必要的资源分配信息和状态
pub struct BankersAlgorithm {
    available: Vec<i32>,      // 当前可用的各类资源数量
    max: Matrix,              // 每个进程对各类资源的最大需求
    allocation: Matrix,       // 每个进程当前已分配的各类资源数量
    need: Matrix,             // 每个进程还需要的各类资源数量
    num_processes: usize,     // 进程总数
    num_resources: usize,     // 资源类型总数
}

impl BankersAlgorithm {
    /// 创建新的银行家算法实例
    /// 参数：
    /// - available: 可用资源向量
    /// - max: 最大需求矩阵
    /// - allocation: 当前分配矩阵
    pub fn new(
        available: Vec<i32>,
        max: Matrix,
        allocation: Matrix,
    ) -> Self {
        let num_processes = max.rows;
        let num_resources = max.cols;
        let mut need = Matrix::new(num_processes, num_resources);

        // 计算初始的需求矩阵：need = max - allocation
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
    /// 使用银行家算法的安全性检查方法
    /// 返回一个元组，包含是否安全的布尔值和安全序列
    pub fn is_safe(&self) -> (bool, Vec<usize>) {
        let mut work = self.available.clone();    // 工作向量，初始等于 available
        let mut finish = vec![false; self.num_processes];  // 记录进程是否能够完成
        let mut safe_sequence = Vec::new();       // 存储安全序列

        // 重复检查直到所有进程都被处理或无法继续
        for _ in 0..self.num_processes {
            println!("{} 迭代开始。Work: {:?}, Finish: {:?}", "→".blue(), work, finish);
            let mut found = false;
            
            // 遍历所有进程，寻找可以安全分配资源的进程
            for i in 0..self.num_processes {
                if !finish[i] && self.can_allocate(&work, i) {
                    // 模拟进程完成并释放资源
                    for j in 0..self.num_resources {
                        work[j] += self.allocation.data[i][j];
                    }
                    finish[i] = true;
                    safe_sequence.push(i);
                    found = true;
                    
                    // 打印当前状态信息
                    println!("{} 进程 {} 被加入安全序列。Finish: {:?}", "→".green(), i, finish);
                    println!("其之前的 Allocation: {:?}, Need: {:?}, 被释放。", self.allocation.data[i], self.need.data[i]);
                    println!("现在的 Work 向量: {:?}", work);
                    break;
                } else if !finish[i] {
                    // 打印无法分配的原因
                    println!("{} 进程 {} 无法分配。Work: {:?}, Finish: {:?}", "→".red(), i, work, finish);
                    println!("因为其 Need: {:?} 但当前可用资源只有: {:?}", self.need.data[i], work);
                }
            }
            if !found {
                println!("{} 本次迭代中未找到安全进程。", "→".red());
                break;
            }
        }

        (finish.iter().all(|&x| x), safe_sequence)
    }

    /// 检查是否可以为指定进程分配资源
    fn can_allocate(&self, work: &Vec<i32>, process: usize) -> bool {
        (0..self.num_resources).all(|j| self.need.data[process][j] <= work[j])
    }

    /// 处理资源请求
    /// 返回是否成功分配资源
    pub fn request_resources(&mut self, process: usize, request: &Vec<i32>) -> bool {
        // 验证请求的合法性
        if !self.is_request_valid(process, request) {
            println!("{} 请求超过最大声明", "错误：".red());
            return false;
        }

        // 检查当前可用资源是否足够
        if !self.has_sufficient_resources(request) {
            println!("{} 资源不足", "错误：".red());
            return false;
        }

        // 尝试分配资源
        self.try_allocation(process, request);

        // 检查分配后的安全性
        let (is_safe, _) = self.is_safe();

        if is_safe {
            println!("{} 资源分配成功", "成功：".green());
            true
        } else {
            // 如果不安全，回滚分配
            self.rollback_allocation(process, request);
            println!("{} 分配会导致不安全状态", "错误：".red());
            false
        }
    }

    /// 验证请求是否不超过声明的最大需求
    fn is_request_valid(&self, process: usize, request: &Vec<i32>) -> bool {
        (0..self.num_resources).all(|j| request[j] <= self.need.data[process][j])
    }

    /// 检查当前可用资源是否足够满足请求
    fn has_sufficient_resources(&self, request: &Vec<i32>) -> bool {
        (0..self.num_resources).all(|j| request[j] <= self.available[j])
    }

    /// 尝试分配资源
    fn try_allocation(&mut self, process: usize, request: &Vec<i32>) {
        for j in 0..self.num_resources {
            self.available[j] -= request[j];
            self.allocation.data[process][j] += request[j];
            self.need.data[process][j] -= request[j];
        }
    }

    /// 回滚资源分配
    fn rollback_allocation(&mut self, process: usize, request: &Vec<i32>) {
        for j in 0..self.num_resources {
            self.available[j] += request[j];
            self.allocation.data[process][j] -= request[j];
            self.need.data[process][j] += request[j];
        }
    }

    /// 打印当前系统状态
    /// 包括可用资源和三个关键矩阵的详细信息
    pub fn print_state(&self) {
        println!("\n{}", "当前系统状态:".yellow().bold());
        
        // 创建并打印可用资源表格
        let mut available_table = Table::new();
        available_table.set_format(*format::consts::FORMAT_BOX_CHARS);
    
        // 添加表头
        let mut header_cells = vec![Cell::new("资源号").style_spec("b")];
        for i in 0..self.available.len() {
            header_cells.push(Cell::new(&format!("资源 {}", i)).style_spec("b"));
        }
        available_table.add_row(Row::new(header_cells));
    
        // 添加可用资源数据行
        let mut data_cells = vec![Cell::new("可用资源数")];
        data_cells.extend(self.available.iter().map(|x| Cell::new(&x.to_string())));
        available_table.add_row(Row::new(data_cells));
        available_table.printstd();
    
        // 创建进程矩阵表格
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);
        
        // 添加矩阵标题行
        let mut header1 = vec![Cell::new("进程号 / 矩阵").style_spec("b")];
        header1.extend(vec![
            Cell::new("Maximum 矩阵").style_spec("bc"),
            Cell::new("Allocation 矩阵").style_spec("bc"),
            Cell::new("Need 矩阵").style_spec("bc"),
        ]);
        table.add_row(Row::new(header1));
    
        // 添加资源编号行
        let mut header2 = vec![Cell::new("进程号 / 资源号").style_spec("b")];
        let resource_headers = (0..self.available.len())
            .map(|i| format!(" {:3}", format!("R{}", i)))
            .collect::<Vec<String>>()
            .join("");
        
        header2.extend(vec![
            Cell::new(&resource_headers).style_spec("b"),
            Cell::new(&resource_headers).style_spec("b"),
            Cell::new(&resource_headers).style_spec("b"),
        ]);
        table.add_row(Row::new(header2));
    
        // 添加每个进程的数据行
        for i in 0..self.max.rows {
            let max_str = self.max.data[i].iter()
                .map(|x| format!("{:3}", x))
                .collect::<Vec<String>>()
                .join(" ");
            
            let allocation_str = self.allocation.data[i].iter()
                .map(|x| format!("{:3}", x))
                .collect::<Vec<String>>()
                .join(" ");
            
            let need_str = self.need.data[i].iter()
                .map(|x| format!("{:3}", x))
                .collect::<Vec<String>>()
                .join(" ");
    
            table.add_row(Row::new(vec![
                Cell::new(&format!("进程 {}", i)),
                Cell::new(&max_str),
                Cell::new(&allocation_str),
                Cell::new(&need_str),
            ]));
        }
    
        table.printstd();
        println!();
    }
}
