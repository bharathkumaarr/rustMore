fn main(){
    let my_string = String::from("hello, world");
    println!(" the length of the string is {}", strlen(my_string));
}
fn strlen(s: String) -> usize {
    s.chars().count()
}
