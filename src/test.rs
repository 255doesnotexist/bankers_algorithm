#[cfg(test)]
mod tests {
    use crate::matrix::Matrix;
    use crate::bankers_algorithm::BankersAlgorithm;

    #[test]
    fn test_basic_safe_state() {
        let available: Vec<i32> = vec![3, 3, 2];
        let max = Matrix::from_vec(vec![
            vec![5, 4, 2],
            vec![3, 2, 2],
            vec![8, 0, 2],
        ]);
        let allocation = Matrix::from_vec(vec![
            vec![0, 1, 0],
            vec![2, 0, 0],
            vec![3, 0, 2],
        ]);

        let banker = BankersAlgorithm::new(available.clone(), max.clone(), allocation.clone());
        
        banker.print_state();
        
        let (is_safe, safe_sequence) = banker.is_safe();
        assert!(is_safe);
        assert_eq!(safe_sequence, vec![1, 0, 2]); // 确保有一个安全序列
    }

    #[test]
    fn test_unsafe_state() {
        let available = vec![0, 1, 1];
        let max = Matrix::from_vec(vec![
            vec![7, 5, 3],
            vec![3, 2, 2],
        ]);
        let allocation = Matrix::from_vec(vec![
            vec![6, 4, 2],
            vec![2, 1, 1],
        ]);

        let banker = BankersAlgorithm::new(available.clone(), max.clone(), allocation.clone());
        
        banker.print_state();
        
        let (is_safe, _) = banker.is_safe();
        assert!(!is_safe);
    }

    #[test]
    fn test_valid_request() {
        let available = vec![3, 3, 2];
        let max = Matrix::from_vec(vec![
            vec![7, 5, 3],
            vec![3, 2, 2],
        ]);
        let allocation = Matrix::from_vec(vec![
            vec![5, 2, 2],
            vec![2, 0, 0],
        ]);

        let mut banker = BankersAlgorithm::new(available.clone(), max.clone(), allocation.clone());
        
        banker.print_state();
        
        let request = vec![1, 0, 1];
        assert!(banker.request_resources(0, &request)); // 确保请求成功

        banker.print_state();
        // 重新检查安全性
        let (is_safe, _) = banker.is_safe();
        assert!(is_safe); // 确保仍然是安全状态
    }

    #[test]
    fn test_invalid_request() {
        let available = vec![3, 3, 2];
        let max = Matrix::from_vec(vec![
            vec![7, 5, 3],
            vec![3, 2, 2],
        ]);
        let allocation = Matrix::from_vec(vec![
            vec![0, 1, 0],
            vec![2, 0, 0],
        ]);

        let mut banker = BankersAlgorithm::new(available.clone(), max.clone(), allocation.clone());
        
        banker.print_state();
        let request = vec![8, 0, 0]; // 超过最大需求
        assert!(!banker.request_resources(0, &request)); // 确保请求失败
    }
}
