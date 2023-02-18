#[cfg(test)]
mod tests {
    use crate::trait_me::doo::{Girl, Human, Boy};

//    use crate::trait_me::doo::*;

    #[test]
    fn test_girl() {
        let girl : Girl = Girl{};
        assert_eq!(girl.cry(), "I cry like a girl!");
        assert_eq!(girl.run(), "RUN")

    }

    #[test]
    fn test_boy() {
        let boy :Boy  = Boy{};
        assert_eq!(boy.cry(), "Boys dont cry!");
        assert_eq!(boy.run(), "RUN");
        assert_eq!(boy.fight(), "fight");


    }


}
