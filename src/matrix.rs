use std::fmt;

// 定义矩阵结构体，包含数据、行数和列数
#[derive(Debug, Clone)]
pub struct Matrix {
    pub data: Vec<Vec<i32>>,  // 使用二维向量存储矩阵数据
    pub rows: usize,          // 矩阵的行数
    pub cols: usize,          // 矩阵的列数
}

impl Matrix {
    // 创建指定行列数的新矩阵，所有元素初始化为0
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![vec![0; cols]; rows],  // 创建指定大小的二维向量并填充0
            rows,
            cols,
        }
    }

    // 从二维向量创建矩阵
    pub fn from_vec(data: Vec<Vec<i32>>) -> Self {
        let rows = data.len();           // 获取行数
        let cols = data[0].len();        // 获取列数
        Matrix { data, rows, cols }
    }
}

// 实现矩阵的显示格式化特征
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 遍历矩阵的每一行
        for row in &self.data {
            // 遍历行中的每个元素
            for (i, val) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, "\t")?;    // 除第一个元素外，其他元素前添加制表符
                }
                write!(f, "{:>4}", val)?;  // 右对齐输出每个元素，宽度为4
            }
            writeln!(f)?;    // 每行结束后换行
        }
        Ok(())
    }
}