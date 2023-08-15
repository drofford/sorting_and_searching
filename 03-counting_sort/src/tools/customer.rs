use std::fmt;

#[derive(Debug)]
pub struct Customer {
    pub id: String,
    pub num_purchases: i32,
}

impl Customer {
    pub fn new(i: &str, np: i32) -> Self {
        let val = Self { id: String::from(i), num_purchases: np };
        return val;
    }
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.id, self.num_purchases)
    }
}
