use std::io; 


fn main(){
    print_array_in_reverse();
    print_sum_of_array();
    minimum_element();
    maximum_element();
    search_element();
}

fn search_element(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    let n: usize = parts.next().unwrap().trim().parse().unwrap();
    let target: i32 = parts.next().unwrap().trim().parse().unwrap();
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();  
    let mut input_array = input.split_whitespace(); 
    let mut v: Vec<i32> = Vec::new(); 
    for _ in 1..=n {
        v.push(input_array.next().unwrap().parse().unwrap());
    }

    let mut found = false;  

    for i in 1..n {
        if target == v[i]{
            found = true; 
        }  
    }
     
    if found {
        print!("YES"); 
    }else {
        print!("NO"); 
    }
}

fn maximum_element(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n: usize = input.trim().parse().unwrap();
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();  
    let mut input_array = input.split_whitespace(); 
    let mut v: Vec<i32> = Vec::new(); 
    for _ in 1..=n {
        v.push(input_array.next().unwrap().parse().unwrap());
    }

    let mut max_idx: usize = 0; 

    for i in 1..n {
        if v[max_idx] < v[i]{
            max_idx = i; 
        }
        
    }
    print!("{} {}", v[max_idx], max_idx + 1); 

}

fn minimum_element(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n: usize = input.trim().parse().unwrap();
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();  
    let mut input_array = input.split_whitespace(); 
    let mut v: Vec<i32> = Vec::new(); 
    for _ in 1..=n {
        v.push(input_array.next().unwrap().parse().unwrap());
    }

    let mut min_idx: usize = 0; 

    for i in 1..n {
        if v[min_idx] > v[i]{
            min_idx = i; 
        }
        
    }
    print!("{} {}", v[min_idx], min_idx + 1); 

}

fn print_sum_of_array(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n: i32 = input.trim().parse().unwrap();
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();  
    let mut parts = input.split_whitespace(); 

    let mut sum = 0 ;

    for _ in 1..=n {
        let a : i32 = parts.next().unwrap().trim().parse().unwrap(); 
        sum += a ; 
    }

    print!("{}", sum)
}


fn print_array_in_reverse(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let n: usize = input.trim().parse().unwrap();
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();  
    let mut input_array = input.split_whitespace(); 
    // println!("{:?}", input_array)
    let mut v = Vec::new(); 
    for _ in 1..=n {
        v.push(input_array.next().unwrap());
    }

    for i in (0..n).rev() {
        print!("{} ", v.get(i).unwrap())
    }

}

