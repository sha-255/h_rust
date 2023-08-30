// fn main() {
//     let val = print_labaled(12, '♦');
//     print!("|{val}");
// }

// fn print_labaled(value: i32, unit_label: char) -> i32 {
//     println!("labaled is: {value}{unit_label}");
//     value
// }
//-
// fn main() {
//     let mut s = String::from("qwe");
//     s.push_str("|String");
//     println!("{}", &s);
// }
//-
// fn main() {
//     let s1 = String::from("string1");
//     let lenth = calc_lenth(&s1);
//     print!("{}|{}", s1, lenth);
// }

// fn calc_lenth(my_str: &String) -> usize {
//     my_str.len()
// }
//-
// use std::io;

// fn main() {
//     loop {
//         let inp = read();
//         println!("Hello {}!", inp);
//     }
// }

// fn read() -> String {
//     let mut inp = String::new();
//     io::stdin().read_line(&mut inp).expect("Err");
//     String::from(inp.trim())
// }

// fn main() {
//     let st = String::from("qwe");
//     let sb = &st;
//     let mut sn = &st;
//     let binding = sn.to_owned() + &String::from("value");
//     sn = &binding;
//     print!("{}, {}", sb, sn);
// }
// fn main() {
//     let mut hello = String::from("Hello");
//     let len = hello.len();
//     let h_world = world_add(&mut hello);
//     print!("{}|{}", h_world, &hello[..len]);
// }

// fn world_add(string: &mut String) -> String {
//     string.push_str(" world!");
//     string.to_string()
// }
// use std::io;

// fn main() {
//     println!("Please input your name: ");
//     let name = read();
//     println!("Please input your age: ");
//     let age = read();
//     println!("Hello {}! You age is {}", name, age);
// }

// fn read() -> String {
//     let mut inp = String::new();
//     io::stdin()
//         .read_line(&mut inp)
//         .expect("Failed to read line");
//     inp.trim().to_owned()
// }
//___
// fn main() {
//     let string = String::from("value a is correct");
//     let first = get_word(&string, 2);
//     println!("{}", first);
// }

// fn get_word(string: &String, word_index: i32) -> &str {
//     let bytes = string.as_bytes();
//     let mut current_word_index = 0;
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             current_word_index += 1;
//             if current_word_index == word_index {
//                 return &string[..i];
//             }
//         }
//     }
//     &string
// }
//______________________________________________________________________
// fn main() {
//     let string = String::from("Lorem ipsum dolor sit amet, consectetur adipiscing elit.");

//     let a = split_by_space(&string);
//     let b: Vec<&str> = string.split(' ').collect();

//     assert_eq!(a, b);
    
//     for word in a {
//         println!("A: {}", word);
//     }

//     for word in b {
//         println!("B: {}", word);
//     }
// }

// fn split_by_space(s: &String) -> Vec<&str> {
//     let mut result = Vec::new();
//     let mut iterator = 0;
//     while iterator < get_word_count(s) {
//         iterator += 1;
//         result.push(get_word(s, iterator));
//     };
//     result
// }

// fn get_word_count(s: &String) -> u32 {
//     let bytes = s.as_bytes();
//     let mut counter = if s != &String::from("") { 1 } else { 0 };
//     for item in bytes.iter() {
//         if item == &b' ' {
//             counter += 1;
//         }
//     }
//     counter
// }

// fn get_word(split_string: &String, word_index: u32) -> &str {
//     let bytes = split_string.as_bytes();
//     let mut current_word_index = 0;
//     let mut last_space_counter = 0;
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             current_word_index += 1;
//             if current_word_index == word_index {
//                 return &split_string[last_space_counter..i];
//             }
//             last_space_counter = i + 1;
//         }
//     }
//     &split_string[last_space_counter..]
// }
//______________________________________________________________________
// #[derive(Debug)]
// struct User {
//     name: String,
//     age: i32,
//     has: Object,
//     adress: Option<Ip>
// }

// impl User {
//     fn get_penis_size(&self) -> i32 {
//         self.age + 20
//     }

//     fn get_boobs_size(&self) -> i32 {
//         if self.name != String::from("Bogdan") {
//             return 0;
//         }
//         self.age - 14
//     }
// }

// impl User {
//     fn make_cringe() -> String {
//         String::from("Lorem ispum")
//     }
// }

// #[derive(Debug)]
// enum Object {
//     Dildo,
//     Pensil,
//     Penis,
//     Chlen
// }

// #[derive(Debug)]
// enum Ip {
//     V8(u8, u8, u8, u8),
//     V6(String)
// }

// fn main() {
//     let std_v6_ip = Ip::V6(String::from("::1"));
//     let us1 = User {
//         name: String::from("Bogdan"),
//         age: 17,
//         has: Object::Pensil,
//         adress: None
//     };
//     let mut us2 = User {
//         adress: Some(Ip::V8(127, 0, 0, 1)),
//         ..us1
//     };
//     us2.has = Object::Dildo;
//     us2.has = Object::Penis;
//     us2.has = Object::Chlen;
//     match us2.has {
//         Object::Dildo => us2.adress = Some(std_v6_ip),
//         Object::Pensil => us2.age = 7,
//         Object::Penis => { 
//             us2.age += 10; 
//             us2.name.push_str(" Bigpenisov");
//         },
//         Object::Chlen => us2.adress = None,
//     }
//     let us2 = if let Object::Chlen = us2.has {
//         User {
//             age: 18,
//             ..us2
//         }
//     }
//     else {
//         User {
//             age: -3,
//             ..us2
//         }
//     };
//     output(&us2);
// }

// fn output(us: &User) {
//     println!(
//         "Penis size is {:#?} | Cringe: {} | Boops size is: {}", 
//         us.get_penis_size(), 
//         User::make_cringe(), 
//         us.get_boobs_size());
//     dbg!("{}", us);
//     let mut inp = String::from("");
//     std::io::stdin().read_line(&mut inp).expect("Uncorrect!");
// }
//______________________________________________________________________