use std::io;

fn main() {
    hollow_vertical_pyramid();
}

fn hollow_vertical_pyramid(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut m: usize = 1; 
    let mut space: usize = 0; 

    let mut row_stars = String::default(); 

    for  i in 1..=(2*n - 1)  {
        if m >=3 && i < n{
            space += 2;
            // println!("The space is {} from 1", space);  
        }else if m >= 3 && i >= n {
            // println!("The space is {} from 2", space);
            space -= 2; 
        }else {

        }

        if m > 1 {
            row_stars = "* *".to_string(); 
        }else {
            row_stars = "*".to_string(); 
        }

        if i < n {
            row_stars.pop(); 
            row_stars.push_str(&" ".repeat(space));
            row_stars.push('*'); 
            m += 1;
        }else if i == n {
            row_stars.pop(); 
            row_stars.push_str(&" ".repeat(space));
            row_stars.push('*');
            m -= 1;   
        }else {
            if m != 1 {
                row_stars.pop(); 
                row_stars.push_str(&" ".repeat(space));
                row_stars.push('*');
                m -= 1; 
            }
            // row_stars.remove(2); 
        }
        println!("{}", row_stars); 
    }
}

fn binary_pyramid(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    let mut binary = "0".to_string(); 
    for i in 1..=n {

        println!("{}", binary); 

        if i % 2 == 0 {
            binary.insert(0, '0');
        }else {
            binary.insert(0, '1');
        }     
    }
}

fn number_pyramid(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    for i in 1..=n {
        println!("{}", i.to_string().repeat(i as usize)); 

    }
}

fn reverse_pyramid(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    for i in (1..=n).rev() {
        println!("{}", "*".repeat(i as usize)); 

    }
}

fn pyramid(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    for i in 1..=n {
        println!("{}", "*".repeat(i as usize)); 

    }
}


fn hollow_rectangle(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 
    let n= parts.next().unwrap().trim().parse().unwrap(); 
    let m: usize  = parts.next().unwrap().trim().parse().unwrap(); 

    for i in 1..=n {
        if i == 1 || i == n{
            println!("{}", "*".repeat(m)); 
        }else {
            println!("{}{}{}", "*", " ".repeat(m - 2), "*"); 
        }
    }
}

fn hollow_square(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    for i in 1..=n {
        if i == 1 || i == n{
            println!("{}", "*".repeat(n as usize)); 
        }else {
            println!("{}{}{}", "*", " ".repeat(n as usize - 2), "*"); 
        }
    }
}

fn square(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    for _ in 1..=n {
        println!("{}", "*".repeat(n as usize)); 
    }
}

fn m_stars(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace(); 
    let n= parts.next().unwrap().trim().parse().unwrap(); 
    let m: usize = parts.next().unwrap().trim().parse().unwrap(); 

    for _ in 1..=n {
         
        println!("{}", "*".repeat(m))
    }
}

fn two_stars() {
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    for _ in 1..=n {
        println!("**"); 
    }
}