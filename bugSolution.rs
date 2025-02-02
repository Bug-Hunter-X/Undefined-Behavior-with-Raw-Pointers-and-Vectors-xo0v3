fn main() {
    let mut v = vec![1, 2, 3];
    // Correct approach: use references
    *v.get_mut(0).unwrap() = 10; // Safe and prevents undefined behavior
    println!("v: {:?}", v);
}