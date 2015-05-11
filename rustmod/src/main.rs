mod dummy;

fn main() {
    println!("[main] try to call function in \\dummy.rs\\.");
    dummy::private_function(); // definded in dummy.rs
}
