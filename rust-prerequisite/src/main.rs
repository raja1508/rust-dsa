use std::println;

fn main() {
    let x = 1; 
    let sum = |y| x + y; 
    assert_eq!(3, sum(2));
    println!("{}",x); 
    let query = |x| x + 2; 
    let mut casher = Casher::new(query); 
    let res = casher.value(2);
    println!("Casher value: {}", res); 
    let res = casher.value(4); 
    println!("Casher value: {}", res);  
    
}

struct Casher<T> where T: Fn(u32) -> u32 {
    query: T,
    value: Option<u32>
}


impl<T> Casher<T> where T: Fn(u32) -> u32 {
    fn new(query: T)-> Casher<T>{
        Casher {
            query,
            value: None
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v, 
            None => {
                let v = (self.query)(arg); 
                self.value = Some(v); 
                v
            }
        }
    }
}