use std::{collections::HashSet, io}; 
fn main() {
    // binary_search();
    // search_queries();
    // search_queries_2();
    // first_and_last_occurence();
    // frequency_of_element();
    frequency_of_element_less_than_or_equal_to_k();
}

fn frequency_of_element_less_than_or_equal_to_k() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let n: u64 = parts.next().unwrap().parse().unwrap(); 
    let q: u64 = parts.next().unwrap().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace(); 

    let mut v: Vec<i64> = Vec::new(); 
    let mut set = HashSet::new();  
    for _ in 0..n {
        let value = parts.next().unwrap().parse().unwrap(); 
        v.push(value);
        set.insert(value);         
    }

    v.sort();

    let mut v2 = Vec::new(); 

    for i in 0..v.len(){
        let t = v[i]; 
        let mut first_index = -1; 
        let mut last_index = - 1; 

        if let None = set.get(&t) {
            continue;
        }else{
            set.remove(&t); 
        }
        
        let mut start = 0; 
        let mut end: i64  = (v.len() - 1) as i64 ; 
        
        while start <= end{
            let mid = start + (end - start) / 2 ; 
            // println!("{} {}", start, end); 
            if v[mid as usize] == t {
                first_index = mid + 1; 
                end = mid - 1 ; 
            }
            if v[mid as usize] > t{
                end = mid - 1; 
            }else if v[mid as usize] < t{
                start = mid + 1; 
            }
        }
    
        let mut start = 0; 
        let mut end: i64  = (v.len() - 1) as i64 ; 
        
        while start <= end{
            let mid = start + (end - start) / 2 ; 
            
            if v[mid as usize] == t {
                last_index = mid + 1; 
                start = mid + 1;  
            }
            if v[mid as usize] > t{
                end = mid - 1; 
            }else if v[mid as usize] < t {
                start = mid + 1; 
            }
        }
        v2.push(last_index - first_index + 1);

    }

    v2.sort();

    let mut v3 = Vec::new(); 
    let mut sum = 0; 
    for i in 0..v2.len() {
        sum += v2[i];
        v3.push(sum); 
    }
 
    for _ in 0..q {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
        
        let k: i64 = input.trim().parse().unwrap();
        let mut start = 0; 
        let mut end: i64  = (v.len() - 1) as i64 ; 
        let mut index = 0; 
        
        while start <= end{
            let mid = start + (end - start) / 2 ; 
            // println!("{} {}", start, end); 
            if v[mid as usize] == k {
                index = mid; 
                end = mid - 1;  
            }
            if v[mid as usize] > k{
                end = mid - 1; 
            }else if v[mid as usize] < k{
                start = mid + 1; 
            }
        }

        println!("{}", end); 
        if end as usize == v2.len() {
             println!("{}", v3[index as usize]); 
        }else {
            println!("{}", v3[v3.len() - 1] - v3[end as usize] ); 
        }
    
    

    }
}

fn frequency_of_element() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let n: u64 = parts.next().unwrap().parse().unwrap(); 
    let k: u64 = parts.next().unwrap().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace(); 

    let mut v: Vec<i64> = Vec::new();  
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }

    v.sort();

    for _ in 0..k {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
        
        let t: i64 = input.trim().parse().unwrap(); 
        let mut first_index = -1; 
        let mut last_index = - 1; 
        
        let mut start = 0; 
        let mut end: i64  = (v.len() - 1) as i64 ; 
        
        while start <= end{
            let mid = start + (end - start) / 2 ; 
            // println!("{} {}", start, end); 
            if v[mid as usize] == t {
                first_index = mid + 1; 
                end = mid - 1 ; 
            }
            if v[mid as usize] > t{
                end = mid - 1; 
            }else if v[mid as usize] < t{
                start = mid + 1; 
            }
        }
        

        let mut start = 0; 
        let mut end: i64  = (v.len() - 1) as i64 ; 
        
        while start <= end{
            let mid = start + (end - start) / 2 ; 
            
            if v[mid as usize] == t {
                last_index = mid + 1; 
                start = mid + 1;  
            }
            if v[mid as usize] > t{
                end = mid - 1; 
            }else if v[mid as usize] < t {
                start = mid + 1; 
            }
        }
        
        if first_index == -1 {
            println!("{}", 0); 
        }else {
            println!("{}", last_index - first_index + 1)
        }
    }
}

fn first_and_last_occurence() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let n: u64 = parts.next().unwrap().parse().unwrap(); 
    let k: u64 = parts.next().unwrap().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace(); 

    let mut v: Vec<i64> = Vec::new();  
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }

    for _ in 0..k {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
        
        let t: i64 = input.trim().parse().unwrap(); 
        let mut first_index = -1; 
        let mut last_index = - 1; 
        
        let mut start = 0; 
        let mut end: i64  = (v.len() - 1) as i64 ; 
        
        while start <= end{
            let mid = start + (end - start) / 2 ; 
            // println!("{} {}", start, end); 
            if v[mid as usize] == t {
                first_index = mid + 1; 
                end = mid - 1 ; 
            }
            if v[mid as usize] > t{
                end = mid - 1; 
            }else if v[mid as usize] < t{
                start = mid + 1; 
            }
        }
        

        let mut start = 0; 
        let mut end: i64  = (v.len() - 1) as i64 ; 
        
        while start <= end{
            let mid = start + (end - start) / 2 ; 
            
            if v[mid as usize] == t {
                last_index = mid + 1; 
                start = mid + 1;  
            }
            if v[mid as usize] > t{
                end = mid - 1; 
            }else if v[mid as usize] < t {
                start = mid + 1; 
            }
        }
        
        if first_index == -1 {
            println!("{}", first_index); 
        }else {
            println!("{} {}", first_index, last_index)
        }
    }
}

fn search_queries_2() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let n: u64 = parts.next().unwrap().parse().unwrap(); 
    let k: u64 = parts.next().unwrap().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace(); 

    let mut v: Vec<i64> = Vec::new();  
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }

    for _ in 0..k {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
        
        let t: i64 = input.trim().parse().unwrap(); 
        let mut index = -1; 
        
        let mut start = 0; 
        let mut end: i64  = (v.len() - 1) as i64 ; 
        
        while start <= end{
            let mid = start + (end - start) / 2 ; 
            
            if v[mid as usize] == t {
                index = mid + 1; 
                break; 
            }
            if v[mid as usize] > t{
                start = mid + 1; 
            }else {
                end = mid - 1; 
            }
        }
        
        println!("{}", index); 
    }
}

fn search_queries() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let n: u64 = parts.next().unwrap().parse().unwrap(); 
    let k: u64 = parts.next().unwrap().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace(); 

    let mut v: Vec<i64> = Vec::new();  
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }

    for _ in 0..k {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
        
        let t: i64 = input.trim().parse().unwrap(); 
        let mut index = -1; 
        
        let mut start = 0; 
        let mut end: i64  = (v.len() - 1) as i64 ; 
        
        while start <= end{
            let mid = start + (end - start) / 2 ; 
            
            if v[mid as usize] == t {
                index = mid + 1; 
                break; 
            }
            if v[mid as usize] > t{
                end = mid - 1; 
            }else {
                start = mid + 1; 
            }
        }
        
        println!("{}", index); 
    }
}

fn binary_search() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let n = input.trim().parse().unwrap(); 

    if n == 0 {
        println!("NO");
        return;
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace(); 

    let mut v: Vec<i64> = Vec::new();  
    for _ in 0..n {
        v.push(parts.next().unwrap().parse().unwrap());
    }

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let t: i64 = input.trim().parse().unwrap(); 
    let mut flag = false; 

    let mut start = 0; 
    let mut end: i64  = (v.len() - 1) as i64 ; 

    while start <= end{
        let mid = start + (end - start) / 2 ; 

        if v[mid as usize] == t {
            flag = true; 
            break; 
        }
        if v[mid as usize] > t{
            end = mid - 1; 
        }else {
            start = mid + 1; 
        }
    }

    if flag {
        println!("YES")
    }else{
        println!("NO")
    }
}