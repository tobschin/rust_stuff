/*
Trait ist im endeffekt ein interface, wie man es aus Java kennt und sagt uns welche methoden wir implementierten sollen
 */

pub trait Human {

    fn cry(&self) -> String;
    fn run(&self) -> String {
        return String::from("RUN");
    }
}

pub struct Girl {}

impl Human for Girl {
    // Methoden fuer Human
     fn cry(&self) -> String {
        return String::from("I cry like a girl!")
    }
}


pub struct Boy{}


impl Human for Boy {
    // Methoden fuer Human
    fn cry(&self) -> String {
        return String::from("Boys dont cry!")
    }
}

impl Boy{
    pub(crate) fn fight(&self) -> String {
        return String::from("fight");
    }
}
