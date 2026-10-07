fn main(){
    // linear in linear data structure refers to the way the data is accessed 
    // stack => LIFO(stack of books) => ability to reverse the order of items
    let mut s = Stack::new(); 
    s.push(1); s.push(2); s.push(3);
    println!("size: {}, {:?}", s.size(), s);
    println!("pop {:?}, size {}",s.pop().unwrap(), s.size());
    println!("empty: {}, {:?}", s.is_empty(), s);
    s.clear();
    println!("{:?}", s);
}

#[derive(Debug)]
pub struct Stack<T> {
    size: usize,
    data: Vec<T>
}

impl<T> Stack<T> {
    pub fn new() -> Self{
        Stack { 
            size: 0,
            data: Vec::new(), 
         }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn clear(&mut self){
        self.size = 0; 
        self.data.clear();
    }

    pub fn push(&mut self, value: T){
        self.data.push(value);
        self.size += 1; 
    }

    pub fn pop(&mut self) -> Option<T>{
        if self.size == 0 {
            return None; 
        }
        self.size -= 1; 
        self.data.pop() 
    }

    pub fn peek(&self) -> Option<&T>{
        if self.size == 0 {
            return None; 
        }
        self.data.get(self.size - 1)
    }

    pub fn into_iter(self) -> IntoIter<T>{
        IntoIter(self)
    }
}


pub struct IntoIter<T>(Stack<T>); 
impl<T> Iterator for IntoIter<T>{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty(){
            self.0.size -= 1; 
            self.0.pop()
        }else{
            None
        }
    }
}