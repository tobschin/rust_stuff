pub fn foo() -> String{
    return String::from("FOOOOO");


}

pub fn fooo() -> & 'static str{
    let l :&str = "leberkas";
    return l;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        assert_eq!(foo(), "FOOOOO");
    }

    #[test]
    fn test_fooo() {assert_eq!(fooo(), "leberkas");}

}
