pub mod matrix_operations {
    /// Multiply two DMatries
    pub fn mul_matrix(
        mat_one: nalgebra::DMatrix<f64>,
        mat_two: nalgebra::DMatrix<f64>
    ) -> nalgebra::DMatrix<f64> {
        return mat_one * mat_two;
    }
    /// Add two DMatries
    pub fn add_matrix(
        mat_one: nalgebra::DMatrix<f64>,
        mat_two: nalgebra::DMatrix<f64>
    ) -> nalgebra::DMatrix<f64> {
        return mat_one + mat_two;
    }
    /// Sub two DMatries
    pub fn sub_matrix(
        mat_one: nalgebra::DMatrix<f64>,
        mat_two: nalgebra::DMatrix<f64>
    ) -> nalgebra::DMatrix<f64> {
        return mat_one - mat_two;
    }
    /// Get the dot Product of matrices
    pub fn dot_product_matrix(
        mat_one: nalgebra::DMatrix<f64>,
        mat_two: nalgebra::DMatrix<f64>
    ) -> f64 {
        return mat_one.dot(&mat_two);
    }
    /// Get the abs of a matrix
    pub fn abs_matrix(mat_one: nalgebra::DMatrix<f64>) -> nalgebra::DMatrix<f64> {
        return mat_one.abs();
    }
    /// Get the max num of a matrix
    pub fn max_matrix(mat_one: nalgebra::DMatrix<f64>) -> f64 {
        return mat_one.max();
    }
    /// Get the min num of a matrix
    pub fn min_matrix(mat_one: nalgebra::DMatrix<f64>) -> f64 {
        return mat_one.min();
    }

    /// Transpose a matrix
    pub fn transpose_matrix(mat_one: nalgebra::DMatrix<f64>) -> nalgebra::DMatrix<f64> {
        return mat_one.transpose();
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_mul_two_square_two_mat() {
        let vec_one = vec![2.0, 2.0, 4.0, 4.0];
        let vec_two = vec![3.0, 3.0, 4.0, 4.0];
        let mat_one = nalgebra::DMatrix::from_vec(2, 2, vec_one);
        let mat_two = nalgebra::DMatrix::from_vec(2, 2, vec_two);
        let res_matrix = super::matrix_operations::mul_matrix(mat_one, mat_two);
        assert_eq!(res_matrix.data.as_vec(), &vec![18.0, 18.0, 24.0, 24.0]);
    }

    #[test]
    fn test_mul_three_square_three_mat() {
        let res_matrix = super::matrix_operations::mul_matrix(
            nalgebra::DMatrix::from_vec(3, 3, vec![3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 5.0, 5.0, 5.0]),
            nalgebra::DMatrix::from_vec(3, 3, vec![3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 5.0, 5.0, 5.0])
        );
        assert_eq!(
            res_matrix.data.as_vec(),
            &vec![36.0, 36.0, 36.0, 48.0, 48.0, 48.0, 60.0, 60.0, 60.0]
        );
    }
    #[test]
    fn test_mul_tousand_square_tousand_mat() {
        let m1: Vec<f64> = (0..1000000).map(|x| x as f64).collect();
        let m2: Vec<f64> = (0..1000000).map(|x| x as f64).collect();
        let res_matrix = super::matrix_operations::mul_matrix(
            nalgebra::DMatrix::from_vec(1000, 1000, m1),
            nalgebra::DMatrix::from_vec(1000, 1000, m2)
        );
        assert_eq!(res_matrix.data.as_vec().len(), 1000000);
    }
    #[test]
    fn test_add_three_square_mat() {
        let res_matrix = super::matrix_operations::add_matrix(
            nalgebra::DMatrix::from_vec(3, 3, vec![3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 5.0, 5.0, 5.0]),
            nalgebra::DMatrix::from_vec(3, 3, vec![3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 5.0, 5.0, 5.0])
        );
        assert_eq!(res_matrix.data.as_vec(), &vec![6.0, 6.0, 6.0, 8.0, 8.0, 8.0, 10.0, 10.0, 10.0]);
    }
    #[test]
    fn test_add_two_square_two_mat() {
        let mat_one = nalgebra::DMatrix::from_vec(2, 2, vec![2.0, 2.0, 4.0, 4.0]);
        let mat_two = nalgebra::DMatrix::from_vec(2, 2, vec![3.0, 3.0, 4.0, 4.0]);
        let res_matrix = super::matrix_operations::add_matrix(mat_one, mat_two);
        assert_eq!(res_matrix.data.as_vec(), &vec![5.0, 5.0, 8.0, 8.0]);
    }
    #[test]
    fn test_sub_two_square_two_mat() {
        let mat_one = nalgebra::DMatrix::from_vec(2, 2, vec![2.0, 2.0, 4.0, 4.0]);
        let mat_two = nalgebra::DMatrix::from_vec(2, 2, vec![3.0, 3.0, 4.0, 4.0]);
        let res_matrix = super::matrix_operations::sub_matrix(mat_one, mat_two);
        assert_eq!(res_matrix.data.as_vec(), &vec![-1.0, -1.0, 0.0, 0.0]);
    }
    #[test]
    fn test_dot_product_three_square_mat() {
        let res = super::matrix_operations::dot_product_matrix(
            nalgebra::DMatrix::from_vec(3, 3, vec![3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 5.0, 5.0, 5.0]),
            nalgebra::DMatrix::from_vec(3, 3, vec![3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 5.0, 5.0, 5.0])
        );
        assert_eq!(res, 150.0);
    }
    #[test]
    fn test_abs_mat() {
        let res_matrix = super::matrix_operations::abs_matrix(
            nalgebra::DMatrix::from_vec(
                3,
                3,
                vec![-3.0, -3.0, -3.0, -4.0, -4.0, -4.0, -5.0, -5.0, -5.0]
            )
        );
        assert_eq!(res_matrix.data.as_vec(), &vec![3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 5.0, 5.0, 5.0]);
    }
    #[test]
    fn test_max_mat() {
        let res = super::matrix_operations::max_matrix(
            nalgebra::DMatrix::from_vec(
                3,
                3,
                vec![100.0, 20.0, 40.0, 10.0, 12.0, 13.0, 17.0, 10.0, 5.0]
            )
        );
        assert_eq!(res, 100.0);
    }

    #[test]
    fn test_min_mat() {
        let res = super::matrix_operations::min_matrix(
            nalgebra::DMatrix::from_vec(
                3,
                3,
                vec![100.0, 20.0, 40.0, 10.0, 12.0, 13.0, 17.0, 10.0, 5.0]
            )
        );
        assert_eq!(res, 5.0);
    }

    #[test]
    fn test_transpose_three_square_three_mat() {
        let res_mat = super::matrix_operations::transpose_matrix(
            nalgebra::DMatrix::from_vec(
                3,
                3,
                vec![100.0, 20.0, 40.0, 10.0, 12.0, 13.0, 17.0, 10.0, 5.0]
            )
        );
        assert_eq!(
            res_mat.data.as_vec(),
            &vec![100.0, 10.0, 17.0, 20.0, 12.0, 10.0, 40.0, 13.0, 5.0]
        );
    }
}