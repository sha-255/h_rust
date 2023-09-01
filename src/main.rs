use h_rust::{
    user::*, 
    user_output::output
};

fn main() {
    let std_v6_ip = Ip::V6(String::from("::1"));
    let us1 = User {
        name: String::from("Bogdan"),
        age: 17,
        has: Object::Pensil,
        adress: None
    };
    let mut us2 = User {
        adress: Some(Ip::V8(127, 0, 0, 1)),
        ..us1
    };
    us2.has = Object::Phone;
    us2.has = Object::NetBook;
    us2.has = Object::Socks;
    match us2.has {
        Object::Phone => us2.adress = Some(std_v6_ip),
        Object::Pensil => us2.age = 7,
        Object::NetBook => { 
            us2.age += 10; 
            us2.name.push_str(" Bigpenisov");
        },
        Object::Socks => us2.adress = None,
    }
    let us2 = if let Object::Socks = us2.has {
        User {
            age: 18,
            ..us2
        }
    }
    else {
        User {
            age: -3,
            ..us2
        }
    };
    output(&us2);
}
