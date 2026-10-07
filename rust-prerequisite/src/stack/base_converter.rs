use crate::Stack; 

pub fn converting_decimal_numbers_to_hex(mut n: u32){
    let digits = ['0', '1', '2', '3', '4', '5', '6', '7',
    '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
    
    let mut stack = Stack::new(); 
    while n > 0 {
        stack.push(n % 16);
        n /= 16; 
    }
    
    
    let mut hex_str = String::new(); 
    while !stack.is_empty() {
        let hex = stack.pop().unwrap();
        let ch = digits[hex as usize]; 
        hex_str.push(ch); 
    }
    
    println!("{}", hex_str)
}

pub fn converting_decimal_numbers_to_binary(n:u32){
    divide_by_two(n); 
}

pub fn converting_decimal_numbers_to_octal(n:u32){
    divide_by_eight(n);
}

pub fn divide_by_eight(mut n:u32){
    let mut stack = Stack::new(); 
    while n > 0 {
        stack.push(n % 8);
        n /= 8; 
    }


    let mut octal_str = String::new(); 
    while !stack.is_empty() {
        let octal = stack.pop().unwrap();
        octal_str.push_str(&octal.to_string()); 
    }

    println!("{}", octal_str)
}

pub fn divide_by_two(mut n:u32){
    let mut stack = Stack::new(); 
    while n > 0 {
        stack.push(n % 2);
        n /= 2; 
    }


    let mut binary_str = String::new(); 
    while !stack.is_empty() {
        let binary = stack.pop().unwrap();
        binary_str.push_str(&binary.to_string()); 
    }
    
    println!("{}", binary_str)
}
