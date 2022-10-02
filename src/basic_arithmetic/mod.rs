//! Collection of basic arithmetic functions
pub mod basic_arithmetic{
    // Add two numbers
    pub fn add_num(a:f64,b:f64)->f64{
        return a+b;
    }

    // Subtract two numbers
    pub fn sub_num(a:f64,b:f64)->f64{
        return a-b;
    }

    // Multiply two numbers
    pub fn mul_num(a:f64,b:f64)->f64{
        return a*b;
    }

    // Divide two numbers
    pub fn div_num(a:f64,b:f64)->f64{
        if b == 0.0 {
            panic!("b cannot be zero");
        }
        return a/b;
    }
    
    // Calculate the modulo of two numbers
    pub fn mod_num(a:f64,b:f64)->f64{
        return a%b;
    }
}

#[cfg(test)]
mod test{
    #[test]
    fn test_add_num_one(){
        assert_eq!(super::basic_arithmetic::add_num(10.0, 20.0),30.0);
    }

    #[test]
    fn test_add_num_two(){
        assert_eq!(super::basic_arithmetic::add_num(0.0, 1.0),1.0);
    }

    #[test]
    fn test_add_num_three(){
        assert_eq!(super::basic_arithmetic::add_num(0.0, 5.0),5.0);
    }

    #[test]
    fn test_add_num_four(){
        assert_eq!(super::basic_arithmetic::add_num(3.0, 3.0),6.0);
    }

    #[test]
    fn test_sub_num_one(){
        assert_eq!(super::basic_arithmetic::sub_num(5.0, 2.0),3.0);
    }

    #[test]
    fn test_sub_num_two(){
        assert_eq!(super::basic_arithmetic::sub_num(10.0, 5.0),5.0);
    }

    #[test]
    fn test_sub_num_three(){
        assert_eq!(super::basic_arithmetic::sub_num(100.0, 50.0),50.0);
    }

    #[test]
    fn test_sub_num_four(){
        assert_eq!(super::basic_arithmetic::sub_num(30.0, 20.0),10.0);
    }
    
    #[test]
    fn test_mul_num_one(){
        assert_eq!(super::basic_arithmetic::mul_num(2.0, 3.0),6.0);
    }

    #[test]
    fn test_mul_num_two(){
        assert_eq!(super::basic_arithmetic::mul_num(3.0, 3.0),9.0);
    }

    #[test]
    fn test_mul_num_three(){
        assert_eq!(super::basic_arithmetic::mul_num(5.0, 5.0),25.0);
    }

    #[test]
    fn test_mul_num_four(){
        assert_eq!(super::basic_arithmetic::mul_num(10.0, 10.0),100.0);
    }

    #[test]
    fn test_div_num_one(){
        assert_eq!(super::basic_arithmetic::div_num(10.0, 5.0),2.0);
    }
    #[test]
    fn test_div_num_two(){
        assert_eq!(super::basic_arithmetic::div_num(100.0, 10.0),10.0);
    }

    #[test]
    fn test_div_num_three(){
        assert_eq!(super::basic_arithmetic::div_num(30.0, 15.0),2.0);
    }

    #[test]
    fn test_div_num_four(){
        assert_eq!(super::basic_arithmetic::div_num(5.0, 2.0),2.5);
    }

    #[test]
    #[should_panic]
    fn test_div_num_five(){
        (super::basic_arithmetic::div_num(5.0, 0.0));
    }
    #[test]
    fn test_mod_num_one(){
        assert_eq!(super::basic_arithmetic::mod_num(5.0, 2.0),1.0);
    }

    #[test]
    fn test_mod_num_two(){
        assert_eq!(super::basic_arithmetic::mod_num(4.0, 2.0),0.0);
    }

}