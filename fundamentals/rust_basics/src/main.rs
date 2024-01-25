mod variables;
//use variables::var::mutability_test;

use variables::var::*;
fn main() {
    println!("Methods from variables/var.rs");
    mutability_test();
    shado_var();
    shado_benifit();
    check_type();
    mutable_const();
    is_shado_allow_mut();
}
