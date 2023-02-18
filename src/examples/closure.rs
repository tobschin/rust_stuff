pub fn closure_example(v : Vec<i32>) -> Vec<i32> {
    //let mut v: Vec<i32> = vec![1, 2, 3];
    return v.iter().map(|x| {
        println!("{}", *x);
        return *x +1
    }).collect::<Vec<_>>(); // _ sagt: egal welcher typ
    
}

#[cfg(test)]
mod tests {
    use std::vec;
    use super::closure_example;

    #[test]
    fn test_closure_example() {
        assert_eq!(closure_example(vec![1,2,3]), vec![2,3,4]);
    }
}



#[cfg(test)]
mod test_closure {
    use super::*;

    #[test]
    fn test_closure_example() {
        assert_eq!(closure_example(vec![1,2,3]), vec![2,3,4]);
    }
}
