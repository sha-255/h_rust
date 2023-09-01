#[derive(Debug)]
pub struct User {
    pub name: String,
    pub age: i32,
    pub has: Object,
    pub adress: Option<Ip>
}

impl User {
    pub fn get_hand_size(&self) -> i32 {
        self.age + 20
    }
    pub fn get_boody_size(&self) -> i32 {
        if self.name != String::from("Bogdan") {
            return 0;
        }
        self.age - 14
    }
    pub fn stroke() -> String {
        String::from("Lorem ispum")
    }
}

#[derive(Debug)]
pub enum Object {
    Phone,
    Pensil,
    NetBook,
    Socks
}

#[derive(Debug)]
pub enum Ip {
    V8(u8, u8, u8, u8),
    V6(String)
}