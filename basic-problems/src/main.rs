use std::io;

fn main() {
    second_larget_element();
}

fn second_larget_element() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let n: u8 = input.trim().parse().unwrap(); 

    if n == 0 {
        print!("{}", -1);        
        return; 
    }
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let mut v: Vec<u16> = Vec::new(); 

    for _ in 1..=n {
        v.push(parts.next().unwrap().parse().unwrap());
    }

    if v.len() == 1 {
        print!("{}", -1); 
        return;
    }

    for i in 0..v.len(){
        let mut min = i; 
        for j in (i + 1)..v.len(){
            if v[min] > v[j] {
                min = j; 
            }
        }
        let temp = v[i]; 
        v[i] = v[min]; 
        v[min] = temp; 

    }
    if v[v.len() -1] != v[v.len() - 2]{
        print!("{}", v[(n - 2) as usize]); 
    }else{
        print!("{}", -1); 
    }
 
}