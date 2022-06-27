
#[link(name="stuff", kind="static")]
extern "C" {
    pub fn add_numbers(a: ::std::os::raw::c_int, b: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}


fn main() {
    unsafe {
        let add_result: i32 = add_numbers (5_i32, 4_i32);
    
        println!("5 + 4 = {}", add_result);
    }
}