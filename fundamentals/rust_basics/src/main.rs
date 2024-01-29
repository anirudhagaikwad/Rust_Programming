mod variables;
mod datatypes;
//use variables::var::mutability_test;

use variables::var::*;
use datatypes::data_types::*;
fn main() {
    // println!("Methods from variables/var.rs");
    // mutability_test();
    // shado_var();
    // shado_benifit();
    // check_type();
    // mutable_const();
    //is_shado_allow_mut();
    println!("Methods from datatypes/data_types.rs");
    scalar_types_implicit();
    scalar_types_explicit();
    compound_type_tuple();
}
