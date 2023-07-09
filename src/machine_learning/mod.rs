#[cfg(feature = "machine_learning")]
pub mod machine_learning{
    use smartcore::linalg::basic::matrix::DenseMatrix;
    use smartcore::cluster::kmeans::*;
    use smartcore::linalg::basic::arrays::Array2;
    pub fn kmeans(x:Vec<Vec<f64>>,k:i64)->Vec<u8>{
        let x_mat = DenseMatrix::from_2d_vec(&x);
        let kmeans = KMeans::fit(&x_mat, KMeansParameters::default().with_k(k as usize)).unwrap();
        let y_hat: Vec<u8> = kmeans.predict(&x_mat).unwrap();
        return y_hat;
    }
    pub fn pca_decomposition(x:Vec<Vec<f64>>,n_components:i64)->Vec<Vec<f64>>{
        let x_mat = DenseMatrix::from_2d_vec(&x);
        let pca = smartcore::decomposition::pca::PCA::fit(&x_mat, smartcore::decomposition::pca::PCAParameters::default().with_n_components(n_components as usize)).unwrap();
        let pca_reduced = pca.transform(&x_mat).unwrap();
        return convert_dense_matrix_to_vec(pca_reduced);
    }
    pub fn svd_decomposition(x:Vec<Vec<f64>>,n_components:i64)->Vec<Vec<f64>>{
        let x_mat = DenseMatrix::from_2d_vec(&x);
        let svd = smartcore::decomposition::svd::SVD::fit(&x_mat,
            smartcore::decomposition::svd::SVDParameters::default().with_n_components(n_components as usize)).unwrap();
        let svd_reduced = svd.transform(&x_mat).unwrap();
        return convert_dense_matrix_to_vec(svd_reduced);
    }
    pub fn random_forest_classifier(x:Vec<Vec<f64>>,y:Vec<i64>)->Vec<i64>{
        let x_mat = DenseMatrix::from_2d_vec(&x);
        let classifier = smartcore::ensemble::random_forest_classifier::RandomForestClassifier::fit(&x_mat, &y, Default::default()).unwrap();
        let res = classifier.predict(&x_mat).unwrap();
        return res;
    }
    fn convert_dense_matrix_to_vec(input_mat:DenseMatrix<f64>)->Vec<Vec<f64>>{
        let mut res_vec:Box<Vec<Vec<f64>>> = Box::new(Vec::new());
        let mut i = 0;
        for row in input_mat.row_iter(){
            let mut col_vec:Vec<f64> = Vec::new();
            for col in row.iterator(0){
                col_vec.push(*col);
            }
            res_vec.push(col_vec);
            i = i +1;
        }
        return *res_vec;
    }
}
#[cfg(feature = "machine_learning")]
#[cfg(test)]
mod test {
    #[test]
    fn test_kmeans_one() {
        let vec_one = vec![
            vec![10.0,40.0,50.0],
            vec![20.0,30.0,90.0],
            vec![30.0,80.0,120.0],
            vec![20.0,60.0,70.0],
            vec![30.0,80.0,90.0],
            vec![3.0,6.0,10.0],
            vec![12.0,13.0,20.0],
            vec![50.0,300.0,900.0],
        ];
        println!("{:?}", super::machine_learning::kmeans(vec_one,10));
    }
    #[test]
    fn test_pca() {
        let input_mat = vec![
            vec![10.0,40.0,50.0],
            vec![20.0,30.0,90.0],
            vec![30.0,80.0,120.0],
            vec![20.0,60.0,70.0],
            vec![30.0,80.0,90.0],
            vec![3.0,6.0,10.0],
            vec![12.0,13.0,20.0],
            vec![50.0,300.0,900.0],
        ];
        println!("{:?}", super::machine_learning::pca_decomposition(input_mat,3));
    }

    #[test]
    fn test_svd() {
        let input_mat = vec![
            vec![10.0,40.0,50.0],
            vec![20.0,30.0,90.0],
            vec![30.0,80.0,120.0],
            vec![20.0,60.0,70.0],
            vec![30.0,80.0,90.0],
            vec![3.0,6.0,10.0],
            vec![12.0,13.0,20.0],
            vec![50.0,300.0,900.0],
        ];
        println!("{:?}", super::machine_learning::svd_decomposition(input_mat,1));
    }
    #[test]
    fn test_random_forest_classifier(){
        let x = vec![
            vec![10.0,40.0,50.0],
            vec![20.0,30.0,90.0],
            vec![30.0,80.0,120.0],
            vec![20.0,60.0,70.0],
            vec![30.0,80.0,90.0],
            vec![3.0,6.0,10.0],
            vec![12.0,13.0,20.0],
            vec![50.0,300.0,900.0],
        ];

        let y = vec![1,1,1,0,0,0,1,1];
        println!("{:?}", super::machine_learning::random_forest_classifier(x,y));
    }
}