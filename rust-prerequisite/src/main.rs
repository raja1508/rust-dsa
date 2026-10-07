
fn main() {
    //closure
    let query = |x| x ; 
    let mut casher = Casher::new(query); 
    let res = casher.value(2);
    println!("Casher value: {}", res); 
    let res = casher.value(4); 
    println!("Casher value: {}", res);  
    
}

struct Casher<T> where T: Fn(u32) -> u32 {
    //closure
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