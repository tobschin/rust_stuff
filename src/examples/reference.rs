pub fn quadrat(num: &mut i32) {
    *num = (*num) * (*num)
}

#[cfg(test)]
mod test_reference {

    use super::*;

    #[test]
    fn test_quadrat() {
        let mut zahl = 12 as i32;
        quadrat(&mut zahl);
        assert_eq!(144, zahl);
    }
}
