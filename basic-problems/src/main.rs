
fn main() {
    let string = String::from("abcde"); 

    for c in string.to_uppercase().chars() {
        println!("{}", c);
    }

    for i in (0..5).rev() {
        let stars = "*".repeat(i+1);
        // let stars : String = iter::repeat('*').take(i + 1).collect();  
        println!("{}", stars)
    }
}
