struct User {
    active: bool, 
    first_name: String, 
    last_name: String,
    age: u32,
}

fn main(){
    let user1 = User {
        active: true,
        first_name: String::from("bharath kumar"),
        last_name: String::from("reddy"),
        age: 20,
    };
    println!("user is active? : {}", user1.active);
    println!("name of user1: {} {}", user1.first_name, user1.last_name);
}
