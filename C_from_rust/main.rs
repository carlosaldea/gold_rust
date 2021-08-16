#[link(name = "stuff")]
extern "C" {
   fn add_numbers(a: i32, b: i32) -> i32;
}


fn main() {
    unsafe {
        let add_result: i32 = add_numbers (5_i32, 4_i32);
    
        println!("5 + 4 = {}", add_result);
    }

}