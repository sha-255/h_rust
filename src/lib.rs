pub mod user {
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
}

pub mod user_output {
    use crate::user::User;
    
    pub fn output(us: &User) {
        println!(
            "Hand size is {:#?} | Stroke: {} | Body: {}", 
            us.get_hand_size(), 
            User::stroke(),
            us.get_boody_size());
        dbg!("{}", us);
        leave_it_open();
    }
    
    fn leave_it_open() {
        let mut inp = String::from("");
        std::io::stdin().read_line(&mut inp).expect("Uncorrect!");
    }
}