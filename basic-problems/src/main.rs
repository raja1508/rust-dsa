use std::{collections::{HashSet}, io};

fn main() {
    // second_larget_element();
    // push_zeros_to_end();
    // check_array_rotation();
    // rotate_array();
    // sum_of_digits();
    // string_compression();
    // longest_word();
    // diagonal_sum();
    // rotate_image();
    spiral_matrix();

}

fn wall_matrix(
    v : &Vec<Vec<i32>>
){
    let mut start_row = 0; 
    let mut start_col = 0; 
    let mut end_row = v.len() - 1; 
    let mut end_col = v[0].len() - 1;
    
    while start_row <= end_row && start_col <= end_col  {
        let row_length = end_row - start_row + 1; 
        let col_length  =  end_col - start_col + 1; 

        for i in start_col..=end_col{
            let row = start_row; 
            let col = i; 
            print!("{} ", v[row][col]); 
        }

        for i in start_row + 1..=end_row{
            let row = i; 
            let col = end_col; 
            print!("{} ", v[row][col]); 
        }
        if row_length > 1 {
            for i in (start_col..end_col).rev(){
                let row = end_row; 
                let col = i; 
                print!("{} ", v[row][col]); 
            }
        }

        if col_length > 1 {
            for i in (start_row + 1..end_row).rev(){
                let row = i; 
                let col = start_col; 
                print!("{} ", v[row][col]); 
            }
        }



        start_row += 1;
        start_col += 1; 

        if row_length > 1 {
            end_row -= 1; 
        }

        if col_length > 1 {
            end_col -= 1;  
        }



    } 
}

fn spiral_matrix() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();

    let mut parts = input.split_whitespace();  
    let m = parts.next().unwrap().parse().unwrap(); 
    let n = parts.next().unwrap().parse().unwrap(); 

    let mut v: Vec<Vec<i32>> = vec![vec![0; n]; m]; 

    for i in 0..m{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 

        let mut row = input.split_whitespace(); 

        for j in 0..n {
            let element: i32 = row.next().unwrap().parse().unwrap();  
            v[i][j] = element; 
        }
    }

    wall_matrix(&v);
}

fn rotate_image() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let n = input.trim().parse().unwrap(); 

    let mut v = vec![vec![0; n]; n]; 

    for i in 0..n {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 

        let mut row = input.split_whitespace(); 

        for j in 0..n {
            let element: u32 = row.next().unwrap().parse().unwrap();  
            v[i][j] = element; 
        }
    }

    for col in 0..n {
        for row in (0..n).rev() {
            print!("{} ", v[row][col])
        }
        println!()
    }

}

fn diagonal_sum(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let n = input.trim().parse().unwrap(); 

    let mut sum = 0; 

    for i in 0..n {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 

        let mut row = input.split_whitespace(); 

        for j in 0..n {
            if i == j {
                let element: u32 = row.nth(j).unwrap().parse().unwrap(); 
                sum += element; 
            }

        }
    }

    println!("{}", sum); 
}


fn longest_word() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let sentence : String = input.trim().parse().unwrap(); 
    let word_array: Vec<char> = sentence.chars().collect(); 
    let target = ' '; 
    let mut count_array: Vec<u16> = Vec::new(); 
    let mut idx: usize = 0;
    let mut count = 0; 

    for i in 0..word_array.len(){ 
        let ch = word_array[i]; 
        if target != ch{
            count += 1; 
        }else{ 
            count_array.insert(idx, count); 
            count = 0; 
            idx += 1; 
        }

        if i == word_array.len() - 1 {
            count_array.insert(idx, count); 
            count = 0; 
        }
    }
    
    // println!("{:?}", count_array); 
    let mut max = count_array[0]; 

    for i in 1..count_array.len() {
        if max < count_array[i] {
            max = count_array[i]
        }
    }

    println!("{}", max); 
}

fn string_compression() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let t: u8 = input.trim().parse().unwrap(); 

    for _ in 0..t {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
    
        let word: String = input.trim().parse().unwrap(); 

        let word_array: Vec<char> = word.chars().collect(); 

        let mut target = word_array[0]; 
        let mut count: u8 = 1; 
        
        let mut ans = String::new(); 
        ans.push(target);
        
        for i in 1..word_array.len(){ 
            let ch = word_array[i]; 
            if target == ch{
                count += 1; 
            }else{
                if count != 1 {
                    ans.push_str(&count.to_string()); 
                }
                target = ch; 
                count = 1; 
                ans.push(target);
            }

            if i == word_array.len() - 1 {
                if count != 1 {
                    ans.push_str(&count.to_string());
                }
            }
        }

        println!("{}", ans); 
    

    }
}

fn sum_of_digits(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    
    let num: String = input.trim().parse().unwrap();

    let num_array: Vec<char>= num.chars().collect(); 

    let mut sum = 0; 

    for digit in num_array{
        if digit == '0' {
            continue;
        }else {
            let value: u32 = digit.to_string().parse().unwrap();
            sum += value;   
        }
    }

    println!("{}", sum); 
}

fn rotate_array() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let t: u8 = input.trim().parse().unwrap(); 

    for _ in 0..t {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
    
        let n: u32 = input.trim().parse().unwrap(); 
        
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
        let mut parts = input.split_whitespace(); 
        
        let mut v: Vec<i32> = Vec::new(); 
    
        for  _ in 0..n {
            let num = parts.next().unwrap().parse().unwrap();
            v.push(num);        
        } 

        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
    
        let d: u32 = input.trim().parse().unwrap(); 

        let mut v2 = Vec::new(); 
        
        for k in d..n{
            let value = v[k as usize]; 
            v2.push(value);
        }

        for i in 0..d {
            let value = v[i as usize]; 
            v2.push(value);
        }

        for j in 0..v2.len(){
            print!("{} ", v2[j])
        }

        println!(); 
    }
}

fn check_array_rotation() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let t: u8 = input.trim().parse().unwrap(); 

    for _ in 0..t {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
    
        let n: u32 = input.trim().parse().unwrap(); 
        
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
        let mut parts = input.split_whitespace(); 
        
        let mut v: Vec<i32> = Vec::new(); 
    
        for  i in 0..n {
            let num = parts.next().unwrap().parse().unwrap();
            v.push(num);        
        } 


        let mut  k = 0;

        let mut x = v[0]; 

        for i in 1..v.len() {
            if x > v[i] {
                k = i; 
                break; 
            }else{
                x = v[i]; 
            }
        }
        println!("{} ", k)
    }
}

fn push_zeros_to_end() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let t: u8 = input.trim().parse().unwrap(); 

    for _ in 0..t {
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
    
        let n: u32 = input.trim().parse().unwrap(); 
        
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap(); 
        let mut parts = input.split_whitespace(); 
        
        let mut v: Vec<i32> = Vec::new(); 
        let mut count_zeros = 0; 
    
        for _ in 1..=n {
            let num = parts.next().unwrap().parse().unwrap();
            if num == 0 {
                count_zeros += 1; 
            }else {
                v.push(num);
            }
        } 

        for i in 0..v.len() {
            print!("{} ", v[i])
        }

        for _ in 0..count_zeros {
            print!("{} ", 0)
        }
        
        println!()
    }
}

fn second_larget_element() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 

    let n: u8 = input.trim().parse().unwrap(); 

    if n == 0 || n == 1{
        print!("{}", -1);        
        return; 
    }
    
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap(); 
    let mut parts = input.split_whitespace(); 
    // let mut v: Vec<u16> = Vec::new(); 
    let mut set: HashSet<u16> = HashSet::new(); 

    for _ in 1..=n {
        let num = parts.next().unwrap().parse().unwrap();
        set.insert(num); 

    }

    if set.len() == 1 {
        print!("{}", -1); 
        return; 
    }

    let mut v: Vec<u16> = Vec::new(); 

    for value in set{
        v.push(value);
    }

    for i in 0..v.len(){
        let mut min: usize = i; 
        for j in (i + 1)..v.len(){
            if v[min] > v[j] {
                min = j; 
            }
        }
        let temp = v[i]; 
        v[i] = v[min]; 
        v[min] = temp; 
    }

    print!("{}", v[v.len() - 2]); 

 
}