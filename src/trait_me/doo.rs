trait Human {
    fn cry();
}

impl Girl for Human {
    fn cry() -> String {
        return String::from("I cry like a girl!")
    }
}


impl Boy for Human {
    fn cry() -> String {
        return String::from("Boys dont cry!")
    }
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
