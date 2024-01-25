use std::io;
pub fn string_input(){
    println!("Enter String : \n ");
    let mut str_input=String::new();
    io::stdin()
    .read_line(&mut str_input)
    .expect("Failed to read line"); 

}