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