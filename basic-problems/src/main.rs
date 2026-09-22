use std::io; 


fn main(){
//     print_array_in_reverse();
//     print_sum_of_array();
//     minimum_element();
//     maximum_element();
//     search_element();
//     count_occurrences();
    // is_sorted();
    // sort_1();
    // reverse();
    // arrange_number();
    // swap_alternate();
    // duplicate_number();
    // missing_number();
    intersection();
}

fn intersection(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let k: usize = input.trim().parse().unwrap();
    let mut v2 = Vec::new(); 

    for _ in 1..=k*2 {
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
        v2.push(v);
    }

    let mut i = 0; 
    while i < v2.len() {
        let len1 = v2[i].len(); 
        let len2 = v2[i + 1].len(); 
        let max_ans_length = if len1 > len2{
            len2
        }else {
            len1 
        }; 

        let mut ans_length = 0; 

        let mut j = 0; 
        while  j < v2[i].len() && ans_length < max_ans_length {
            let mut count = 0; 

            for k in 0..v2[i + 1].len() {
                if v2[i][j] == v2[i + 1][k]{
                    count += 1; 
                }
                
            }
            
            if count == 1 {
                let ans = v2[i][j];
                ans_length += 1;  
                print!("{} ", ans); 
            }
            j += 1; 
        } 
        println!(); 
        i += 2
     }
        
}



fn duplicate_number(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let k: usize = input.trim().parse().unwrap();
    let mut v2 = Vec::new(); 

    for _ in 1..=k {
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
        v2.push(v);
    }


    for i in 0..k {
        let len = v2[i].len();
        let ans; 
        
        if len == 1 {
            println!("{} ", v2[i][0]); 
            break; 
            
        }
        
        let mut j = 0; 
        while  j < len  {
            let mut count = 0; 
            let target = v2[i][j]; 
            for k in 0..len {
                if target == v2[i][k]{
                    count += 1; 
                }
            }

            if count == 2 {
                ans = v2[i][j]; 
                println!("{} ", ans); 
                break; 
            }
            j += 1; 
        }


        
    }
}

fn missing_number(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let k: usize = input.trim().parse().unwrap();
    let mut v2 = Vec::new(); 

    for _ in 1..=k {
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
        v2.push(v);
    }


    for i in 0..k {
        let len = v2[i].len();
        let ans; 
        
        if len == 1 {
            println!("{} ", v2[i][0]); 
            break; 
            
        }
        
        let mut j = 0; 
        while  j < len  {
            let mut count = 0; 
            let target = v2[i][j]; 
            for k in 0..len {
                if target == v2[i][k]{
                    count += 1; 
                }
            }
            
            if count == 1 {
                ans = v2[i][j]; 
                println!("{} ", ans); 
                break; 
            }


            j += 1; 
        }


        
    }
}

fn swap_alternate(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let k: usize = input.trim().parse().unwrap();
    let mut v2 = Vec::new(); 

    for _ in 1..=k {
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
        v2.push(v);
    }


    for i in 0..k {
        let len = v2[i].len();
        let mut iterations = 0; 
        if len % 2 == 0 {
            iterations = len
        }else {
            iterations = len - 1 
        }
        if len > 2 {

            let mut j = 0; 
            while  j < iterations - 1 {
                v2[i].swap(j, j + 1);
                j += 2; 
            }
        }


        for k in 0..len{
            print!("{} ", v2[i][k])
        }
        println!()

    }
}

fn arrange_number(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let k: usize = input.trim().parse().unwrap();
    let mut v2 = Vec::new(); 

    for _ in 1..=k {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
        let n: usize = input.trim().parse().unwrap();
        let mut v: Vec<i32> = Vec::new(); 
        let mut temp: Vec<i32> = Vec::new();
        for i in 1..=n {
            if i % 2 == 0 {
                temp.push(i as i32);
            }else {
                v.push(i as i32);
            }
        }

        for _ in 1..=temp.len() {
            let element = temp.pop().unwrap(); 
            v.push(element);
        }
        v2.push(v);
    }
    
    for i in 0..k {
        let len = v2[i].len(); 
        for j in 0..len{
            let v = v2[i].clone();
            print!("{} ", v[j]) 
        }
        println!()
         
    }
}

fn reverse() {
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

    let mut start = 0; 
    let mut end = n - 1; 

    while start < end {
        let temp = v[start]; 
        v[start] = v[end]; 
        v[end] = temp;
        start += 1;
        end -= 1;  

    }

    for i in 0..n {
        print!("{} ", v[i])
    }
}


fn sort_1(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let k: usize = input.trim().parse().unwrap();
    let mut v2 = Vec::new(); 

    for _ in 1..=k {
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

        v2.push(v);
    }
    
    for i in 0..k {
        let mut count = 0;  
        let target = 0; 
        let len = v2[i].len(); 
        for j in 0..len{
            let v = v2[i].clone(); 
            if target == v[j]{
                 count += 1; 
            }  
        }
        println!("{}{}", "0 ".repeat(count), "1 ".repeat(len - count) )
    }
}

fn is_sorted(){
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

    let mut is_sorted = true; 
    

    for i in 0..n - 1 {
        if v[i] > v[i + 1]{
             is_sorted = false
        }
        
    }

    if is_sorted {
        print!("YES"); 
    }else {
        print!("NO"); 
    }
    

}

fn count_occurrences(){
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

    let mut count = 0;  

    for i in 0..n {
        if target == v[i]{
            count += 1; 
        }  
    }
     
    print!("{}", count)
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

    for i in 0..n {
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

