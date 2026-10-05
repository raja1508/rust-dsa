use std::{collections::{HashMap, HashSet, hash_map}, io, vec}; 
fn main() {
    // binary_search();
    // search_queries();
    // search_queries_2();
    // first_and_last_occurence();
    // frequency_of_element();
    // frequency_of_element_greater_than_or_equal_to_k();
    // frequency_of_element_between_x_and_y();
    // smallest_element_greater_than_or_equal_to_x();
    // largest_element_less_than_or_equal_to_x();
    // aggressive_cows();
    // painters_partition();
    allocate_minimum_pages();
}

fn is_cool_3(v: &Vec<u64>, time_block: u64, k: u64) -> bool {
    let mut prev_sum = 0;   
    let mut  student_assigned = 1; 

    let mut i = 0; 
    while i < v.len(){
        prev_sum += v[i]; 
        if prev_sum <= time_block {
            i += 1;     
        }else {
            student_assigned += 1;  
            prev_sum = v[i]; 
            i+=1; 
        }

    }

    student_assigned <= k
}

fn allocate_minimum_pages() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let n: u64 = parts.next().unwrap().parse().unwrap(); 
    let k: u64 = parts.next().unwrap().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace(); 

    let mut v: Vec<u64> = Vec::new(); 
    for _ in 0..n {
        let value = parts.next().unwrap().parse().unwrap(); 
        v.push(value);    
    }

    let mut start = *v.iter().max().unwrap(); 
    let mut end = v.iter().sum();
    let mut ans: i64 = -1;  

    while start <= end && k <= v.len() as u64{
        let mid = start + (end - start) / 2; 
        if is_cool_3(&v, mid, k){
            ans = mid as i64; 
            end = mid - 1; 
        }else{
            start = mid + 1; 
        }        
    }

    println!("{}", ans); 
    

}

fn is_cool_2(v: &Vec<u64>, time_block: u64, k: u64) -> bool {
    let mut prev_sum = 0;   
    let mut  painters_assigned = 1; 

    let mut i = 0; 
    while i < v.len(){
        prev_sum += v[i]; 
        if prev_sum <= time_block {
            i += 1;     
        }else {
            painters_assigned += 1;  
            prev_sum = v[i]; 
            i+=1; 
        }

    }

    painters_assigned <= k
}

fn painters_partition() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let n: u64 = parts.next().unwrap().parse().unwrap(); 
    let k: u64 = parts.next().unwrap().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace(); 

    let mut v: Vec<u64> = Vec::new(); 
    for _ in 0..n {
        let value = parts.next().unwrap().parse().unwrap(); 
        v.push(value);    
    }



    let mut start = *v.iter().max().unwrap(); 
    let mut end = v.iter().sum();
    let mut ans = 1;  



    while start <= end {
        let mid = start + (end - start) / 2; 
        if is_cool_2(&v, mid, k){
            ans = mid; 
            end = mid - 1; 
        }else{
            start = mid + 1; 
        }        
    }

    println!("{}", ans); 
    

}

fn is_cool(v: &Vec<u64>, distance: u64, k: u64) -> bool {
    let mut placed_cows = 1;
    let mut prev_loc = 0; 

    for i in 1..v.len(){
        if v[i] - v[prev_loc] >= distance {
            prev_loc = i; 
            placed_cows += 1; 
        }
    }
    placed_cows >= k
}

fn aggressive_cows() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let n: u64 = parts.next().unwrap().parse().unwrap(); 
    let k: u64 = parts.next().unwrap().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace(); 

    let mut v: Vec<u64> = Vec::new(); 
    for _ in 0..n {
        let value = parts.next().unwrap().parse().unwrap(); 
        v.push(value);    
    }

    v.sort();

    let largest_distance = v[v.len() - 1] - v[0]; 
    let mut start = 0; 
    let mut end = largest_distance;
    let mut ans = 1;  

    while start <= end {
        println!("start: {}", start); 
        println!("end: {}", end); 
        let mid = start + (end - start) / 2; 
        println!("mid: {}", mid); 
        if is_cool(&v, mid, k){
            ans = mid; 
            println!("ans: {}", ans); 
            start = mid + 1 ; 
        }else{
            end = mid - 1; 
        }        
    }

    println!("{}", ans); 
    

}

fn largest_element_less_than_or_equal_to_x() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let n: u64 = parts.next().unwrap().parse().unwrap(); 
    let q: u64 = parts.next().unwrap().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace(); 

    let mut v: Vec<i64> = Vec::new(); 
    for _ in 0..n {
        let value = parts.next().unwrap().parse().unwrap(); 
        v.push(value);        
    }

    v.sort();

 
    for _ in 0..q {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
        
        let x: i64 = input.trim().parse().unwrap();
        let mut start = 0; 
        let mut end: i64  = (v.len() - 1) as i64 ; 
        
        while start <= end{
            let mid = start + (end - start) / 2 ; 
            // println!("{} {}", start, end); 
            if v[mid as usize] == x{
                start = mid + 1;   
            }
            if v[mid as usize] > x{
                end = mid - 1; 
            }else if v[mid as usize] < x{
                start = mid + 1; 
            }
        }

        if end == -1 {
            println!("{}", -1 )
        }else {
            println!("{}", v[end as usize]); 
        }

    }
}

fn smallest_element_greater_than_or_equal_to_x() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let n: u64 = parts.next().unwrap().parse().unwrap(); 
    let q: u64 = parts.next().unwrap().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace(); 

    let mut v: Vec<i64> = Vec::new(); 
    for _ in 0..n {
        let value = parts.next().unwrap().parse().unwrap(); 
        v.push(value);     
    }

    v.sort();

 
    for _ in 0..q {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
        
        let x: i64 = input.trim().parse().unwrap();
        let mut start = 0; 
        let mut end: i64  = (v.len() - 1) as i64 ; 
        
        while start <= end{
            let mid = start + (end - start) / 2 ; 
            // println!("{} {}", start, end); 
            if v[mid as usize] == x{
                end = mid - 1;  
            }
            if v[mid as usize] > x{
                end = mid - 1; 
            }else if v[mid as usize] < x{
                start = mid + 1; 
            }
        }

        if start == v.len() as i64 {
            println!("{}", -1 )
        }else {
            println!("{}", v[start as usize]); 
        }

    }
}

fn frequency_of_element_between_x_and_y() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 

    let n: u64 = parts.next().unwrap().parse().unwrap(); 
    let q: u64 = parts.next().unwrap().parse().unwrap(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace(); 

    let mut v: Vec<i64> = Vec::new(); 
 
    for _ in 0..n {
        let value = parts.next().unwrap().parse().unwrap(); 
        v.push(value);     
    }

    v.sort();

 
    for _ in 0..q {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
        let mut parts = input.split_whitespace(); 
        let x: i64 = parts.next().unwrap().parse().unwrap();
        let y: i64 = parts.next().unwrap().parse().unwrap();
        let mut start = 0; 
        let mut end: i64  = (v.len() - 1) as i64 ;
        let start_idx;  

        
        while start <= end{
            let mid = start + (end - start) / 2 ; 
            // println!("{} {}", start, end); 
            if v[mid as usize] == x {
                end = mid - 1;  
            }
            if v[mid as usize] > x{
                end = mid - 1; 
            }else if v[mid as usize] < x{
                start = mid + 1; 
            }
        }
        start_idx = start; 


        let mut start = 0; 
        let mut end: i64  = (v.len() - 1) as i64 ;
        let end_idx;   
        
        while start <= end{
            let mid = start + (end - start) / 2 ; 
            // println!("{} {}", start, end); 
            if v[mid as usize] == y {
                start = mid + 1;  
            }
            if v[mid as usize] > y{
                end = mid - 1; 
            }else if v[mid as usize] < y{
                start = mid + 1; 
            }
        }
        end_idx = end; 

        if x > y {
            println!("{}", 0);
        }else if start_idx == v.len() as i64 ||  end_idx == v.len() as i64{
            println!("{}", 0); 
        }else{
            println!("{}", end_idx - start_idx + 1); 
        }

    }
}

fn frequency_of_element_greater_than_or_equal_to_k() {
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

 
    for _ in 0..q {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
        
        let k: i64 = input.trim().parse().unwrap();
        let mut start = 0; 
        let mut end: i64  = (v.len() - 1) as i64 ; 
        
        while start <= end{
            let mid = start + (end - start) / 2 ; 
            // println!("{} {}", start, end); 
            if v[mid as usize] == k {
                end = mid - 1;  
            }
            if v[mid as usize] > k{
                end = mid - 1; 
            }else if v[mid as usize] < k{
                start = mid + 1; 
            }
        }

        println!("{}", v.len() as i64 - start); 

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