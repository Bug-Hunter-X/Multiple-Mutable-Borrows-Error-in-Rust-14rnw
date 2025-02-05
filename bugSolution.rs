fn main() {
    let mut x = 5;
    { // Create a new scope to limit the mutable borrow
        let y = &mut x; 
        *y += 1;
    }
    { // Create a new scope to limit the mutable borrow
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x);
}
