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

    pub fn peek_mut(&mut self) -> Option<&mut T>{
        if self.size == 0{
            return None; 
        }

        Some(&mut self.data[self.size - 1])
    }

    pub fn into_iter(self) -> IntoIter<T>{
        IntoIter(self)
    }
    
    pub fn iter<'a>(&'a self) -> Iter<'a, T>{
        let mut iterator =Iter{stack : Vec::new()}; 
        for item in self.data.iter(){
            iterator.stack.push(item);
        }
        iterator
    }

    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T>{
        let mut iterator =IterMut{stack : Vec::new()}; 
        for item in self.data.iter_mut(){
            iterator.stack.push(item);
        }
        iterator
    }
}

pub struct IntoIter<T>(Stack<T>); 
impl<T> Iterator for IntoIter<T>{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> { stack: Vec<&'a T>}
impl <'a, T> Iterator for Iter<'a, T>{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

pub struct IterMut<'a, T> { stack: Vec<&'a mut T>}
impl <'a, T> Iterator for IterMut<'a, T>{
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}
