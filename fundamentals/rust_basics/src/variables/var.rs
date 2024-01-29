// Function demonstrating immutability in Rust
pub fn mutability_test(){
    let x = 30; // Declare an immutable variable x
    println!("cannot assign twice to immutable variable x = {x}");
    // x=40; // Uncommenting this line will result in error: cannot assign twice to immutable variable `x`
}

// Function demonstrating variable shadowing in Rust
pub fn shado_var(){
    let x = 30; // Declare a variable x
    println!("Original value of x = {x}");
    let x = x + 1; // Shadowing the original variable x
    println!("Shadow of x = {x}");
    {
        let x = x + 1; // Shadowing within an inner scope
        println!("The value of x in the inner scope is: {x}");
    }
    println!("Last value of x = {x}");
}

// Function demonstrating the benefits of variable shadowing in Rust
pub fn shado_benifit(){
    let length = "   "; // Declare a string
    println!("String = {length}");
    let length = length.len(); // Shadow the variable with an integer
    println!("Integer = {length}");
}

// Function demonstrating how to check the type of a variable in Rust
pub fn check_type(){
    let x = 10; // Declare a variable
    print_type_of(&x); // Call a function to print the type of x
}

use std::any::type_name;
// Function to print the type of a variable
//generic function in Rust that takes a reference to a value of any type (T) and prints the type of that value
//_: &T  : The underscore (_) is a placeholder for the parameter name, indicating that the value is not used within the function. The parameter is a reference (&) to a value of type T.
pub fn print_type_of<T>(_: &T) {
    println!("type of {}", type_name::<T>()); //call to the type_name function from the std::any module.
    //This function returns a string representation of the type of the provided value or type.
}

// Function demonstrating mutable and constant variables in Rust
pub fn mutable_const(){
    let mut x = 10; // Declare a mutable variable x
    println!("x original value = {x}");
    x = 20; // Modify the value of x
    println!("x change value = {x}");
    // x = "Hello"; // Uncommenting this line would result in an error because x is not mutable
    println!("x change type not allowed in Rust = {x}"); 
    const Y: i32 = 10; // Declare a constant variable Y
    println!("constant Y = {Y}");
    // Shadowing not allowed to constant a variable’s type
}

// Function demonstrating variable shadowing with mutability in Rust
pub fn is_shado_allow_mut() {
    let mut x = "hello"; // Declare a mutable variable x with a string value
    println!("Original value of x = {x} ");
    // Shadowing: Rebind x to the length of the string, creating a new variable x
    let x = x.len(); 
    println!("Shadow value of x = {x}");
}
