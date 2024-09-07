/**
 * Raw pointers are typically used in scenarios where you need to interact with low-level system APIs, 
 * perform manual memory management, or interface with C code. 
 */
use std::slice;

fn main() {

    // ====== Dereferencing a raw pointer ======
    let mut num = 5;

    let r1 = &num as *const i32;    // cast an immutable reference to a raw pointer
    let r2 = &mut num as *mut i32;  // cast a mutable reference to a raw pointer
    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }

    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr(); // access the raw pointer of the slice
    
        assert!(mid <= len);
        
        // rust is not smart enough to know that we are borrowing two non-overlapping slices.
        // it only knows that we are borrowing from the same slice twice.
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    // ===== Using extern Functions to Call External Code =====
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    // this usage of extern does not require `unsafe`
    #[no_mangle] // annotation to tell the Rust compiler not to mangle the name of this function.
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }

    call_from_c();
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // ====== Accessing or Modifying a Mutable Static Variable ======
    // constants vs static variables
    // - Constants are always immutable
    // - Static variables can be mutable, have fixed address in memory
    
    // rust considers mutable static variables to be unsafe because another part of your code might change the data in that variable at any time.
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {COUNTER}");
    }


}
