use crate::matrix::Matrix;
use prettytable::{Table, Row, Cell, format};
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

        for _ in 0..self.num_processes { // 迭代最多进程数量次
            println!("{} 迭代开始。Work: {:?}, Finish: {:?}", "→".blue(), work, finish);
            let mut found = false;
            for i in 0..self.num_processes {
                if !finish[i] && self.can_allocate(&work, i) {
                    // 在检查后模拟完成和资源释放
                    for j in 0..self.num_resources {
                        work[j] += self.allocation.data[i][j];
                    }
                    finish[i] = true;
                    safe_sequence.push(i);
                    found = true;
                    println!("{} 进程 {} 被加入安全序列。Finish: {:?}", "→".green(), i, finish);
                    println!("其之前的 Allocation: {:?}, Need: {:?}, 被释放。", self.allocation.data[i], self.need.data[i]);
                    println!("现在的 Work 向量: {:?}", work);
                    break; // 找到一个安全进程后，移动到下一次迭代
                } else if !finish[i] {
                    println!("{} 进程 {} 无法分配。Work: {:?}, Finish: {:?}", "→".red(), i, work, finish);
                    println!("因为其 Need: {:?} 但当前可用资源只有: {:?}", self.need.data[i], work);
                }
            }
            if !found {
                println!("{} 本次迭代中未找到安全进程。", "→".red());
                break; // 在此迭代中未找到安全进程，退出循环
            }
        }

        (finish.iter().all(|&x| x), safe_sequence)
    }

    fn can_allocate(&self, work: &Vec<i32>, process: usize) -> bool {
        (0..self.num_resources).all(|j| self.need.data[process][j] <= work[j])
    }

    pub fn request_resources(&mut self, process: usize, request: &Vec<i32>) -> bool {
        // 在检查可用资源之前，检查请求是否超过最大声明
        if !self.is_request_valid(process, request) {
            println!("{} 请求超过最大声明", "错误：".red());
            return false;
        }

        if !self.has_sufficient_resources(request) {
            println!("{} 资源不足", "错误：".red());
            return false;
        }

        // 尝试分配
        self.try_allocation(process, request);

        // 检查安全性
        let (is_safe, _) = self.is_safe();

        if is_safe {
            println!("{} 资源分配成功", "成功：".green());
            true
        } else {
            // 回滚
            self.rollback_allocation(process, request);
            println!("{} 分配会导致不安全状态", "错误：".red());
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
        println!("\n{}", "当前系统状态:".yellow().bold());
        
        // Available 向量表格
        let mut available_table = Table::new();
        available_table.set_format(*format::consts::FORMAT_BOX_CHARS);

        // 创建表头
        let mut header_cells = vec![Cell::new("").style_spec("b")];
        for i in 0..self.available.len() {
            header_cells.push(Cell::new(&format!("Resource {}", i)).style_spec("b"));
        }
        available_table.add_row(Row::new(header_cells));

        // 创建数据行
        let mut data_cells = vec![Cell::new("Available 数量")];
        data_cells.extend(self.available.iter().map(|x| Cell::new(&x.to_string())));
        available_table.add_row(Row::new(data_cells));
        available_table.printstd();

        // 矩阵表格
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BOX_CHARS);
        
        // 添加表头
        table.add_row(Row::new(vec![
            Cell::new("进程号").style_spec("b"),
            Cell::new("Max 矩阵").style_spec("b"),
            Cell::new("Allocation 矩阵").style_spec("b"),
            Cell::new("Need 矩阵").style_spec("b"),
        ]));

        // 添加数据行
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
                Cell::new(&format!("P{}", i)),
                Cell::new(&max_str),
                Cell::new(&allocation_str),
                Cell::new(&need_str),
            ]));
        }

        table.printstd();
        println!();
    }
}
