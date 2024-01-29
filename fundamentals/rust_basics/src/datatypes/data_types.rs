// Length	Signed	Unsigned
// 8-bit	    i8	    u8
// 16-bit	    i16	    u16
// 32-bit	    i32	    u32
// 64-bit	    i64	    u64
// 128-bit	    i128	u128
// arch	        isize	usize
//use crate::variables::var::print_type_of;
// Function to demonstrate implicit declaration and information about scalar types
pub fn scalar_types_implicit(){
// Integer types
let i_imp = 55; // Implicitly declared integer variable with value 55
println!("Implicit declaration {}", i_imp); // Print the value of i_imp
print_type_of(&i_imp); // Print the type of i_imp
print_type_size(&i_imp); // Print the size of i_imp

// Floating-point types
let f_imp = 5.2; // Implicitly declared floating-point variable with value 5.2
println!("Implicit declaration {}", f_imp); // Print the value of f_imp
print_type_of(&f_imp); // Print the type of f_imp
print_type_size(&f_imp); // Print the size of f_imp

// Boolean type
let b_imp = false; // Implicitly declared boolean variable with value false
println!("Implicit declaration {}", b_imp); // Print the value of b_imp
print_type_of(&b_imp); // Print the type of b_imp
print_type_size(&b_imp); // Print the size of b_imp

// Character type
let c_imp = 'A'; // Implicitly declared character variable with value 'A'
println!("Implicit declaration {}", c_imp); // Print the value of c_imp
print_type_of(&c_imp); // Print the type of c_imp
print_type_size(&c_imp); // Print the size of c_imp

}

// Function to demonstrate explicit declaration of scalar types
pub fn scalar_types_explicit(){  
// Explicitly declared unsigned 64-bit integer variable with value 30
let i_exp: u64 = 30;
println!("Explicit declaration {}", i_exp); // Print the value of i_exp
print_type_of(&i_exp); // Print the type of i_exp

// Explicitly declared 32-bit floating-point variable with value 5.2
let f_imp: f32 = 5.2;
println!("Explicit declaration {}", f_imp); // Print the value of f_imp
print_type_of(&f_imp); // Print the type of f_imp

// Explicitly declared boolean variable with value true
let b_imp: bool = true;
println!("Explicit declaration {}", b_imp); // Print the value of b_imp
print_type_of(&b_imp); // Print the type of b_imp

// Explicitly declared character variable with the Unicode character '😻'
let c_imp: char = '😻';
println!("Explicit declaration {}", c_imp); // Print the value of c_imp
print_type_of(&c_imp); // Print the type of c_imp
}

// Function to demonstrate compound type - Tuple
pub fn compound_type_tuple() {
    // Declare a tuple named tup1 with elements of types f64, char, and i64
    let tup1: (f64, char, i64) = (5.2, 'A', 65);
    println!("tuple value tup1= ({}, {}, {})", tup1.0, tup1.1, tup1.2);
    print_type_size(&tup1); // Print the size & type of tup1
    // Declare a tuple named tup2 with elements of unspecified types
    let tup2 = (500, 6.4, 1);
    // Destructure the tuple tup2 into variables x, y, and z
    let (x, y, z) = tup2;
    // Print the values of x, y, and z using named variables
    println!("tuple value tup2=({x}, {y}, {z})", x = x, y = y, z = z);
}

// Function to demonstrate compound type - Array
pub fn compound_type_array() {
    let mut a = [1, 2, 3, 4, 5];
    a[0]=8;
    let second = a[1];
}

use std::any::type_name;
// Function to print the type of a variable
//generic function in Rust that takes a reference to a value of any type (T) and prints the type of that value
//_: &T  : The underscore (_) is a placeholder for the parameter name, indicating that the value is not used within the function. The parameter is a reference (&) to a value of type T.
pub fn print_type_of<T>(_: &T) {
    println!("type of {}", type_name::<T>()); //call to the type_name function from the std::any module.
    //This function returns a string representation of the type of the provided value or type.
}

use std::mem;
// Function to print the size of a type in bytes and bits
pub fn print_type_size<T>(_: &T) {
    // Print the size of the type T in bytes
    println!("size of {} in bytes: {}", std::any::type_name::<T>(), std::mem::size_of::<T>());
    // Print the size of the type T in bits (assuming 8 bits per byte)
    println!("size of {} in bits: {}", std::any::type_name::<T>(), std::mem::size_of::<T>() * 8);
}

