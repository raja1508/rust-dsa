use crate::Stack; 
    // Bracket checker -> BODMAS
    // let ba = "(())()"; 
    // let bb = "()(()"; 
    // let res1 = bracket_checker(ba);
    // let res2 = bracket_checker(bb);
    // println!("{ba} balanced:{res1}, {bb} balanced:{res2}");
    // let sa = "(){}[]";
    // let sb = "(){)[}";
    // let res1 = bracket_checker_2(sa);
    // let res2 = bracket_checker_2(sb);
    // println!("sa balanced:{res1}, sb balanced:{res2}");
    // println!("{}", bracket_match('(', ')')); 
    // println!("{}", bracket_match('(', '}')); 
    // println!("{}", bracket_match('(', ']')); 
    // println!("{}", bracket_match('{', '}')); 

    // let sa = "(2+3){func}[abc]"; let sb = "(2+3)*(3-1";
    // let res1 = bracket_checker_3(sa); 
    // let res2 = bracket_checker_3(sb);
    // println!("sa balanced:{res1}, sb balanced:{res2}");

pub fn bracket_checker_3(barcket: &str) -> bool{
    let char_list: Vec<char> = barcket.chars().collect(); 

    let mut index = 0; 
    let mut balance = true; 
    let mut stack = Stack::new(); 

    while index < char_list.len() && balance{
        let ch = char_list[index]; 

        if ch == '(' || ch == '{' || ch == '['{
            stack.push(ch);
        }

        if ch == ')' || ch == '}' || ch == ']' {
            let top = stack.peek().unwrap(); 
            if par_match(*top, ch){
                stack.pop(); 
            }else{
                balance = false; 
            }   
        }
        index += 1; 
    }
    
    balance && stack.is_empty()
}

pub fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}

pub fn bracket_checker_2(barcket: &str) -> bool{
    let char_list: Vec<char> = barcket.chars().collect(); 

    let mut index = 0; 
    let mut balance = true; 
    let mut stack = Stack::new(); 

    while index < char_list.len() && balance{
        let ch = char_list[index]; 

        if ch == '(' || ch == '{' || ch == '['{
            stack.push(ch);
        }else {
            if stack.is_empty(){
                balance = false
            }else{
                let top = stack.peek().unwrap(); 
                if par_match(*top, ch){
                    stack.pop(); 
                }else{
                    balance = false; 
                }
            }
        }
        index += 1; 
    }
    
    balance && stack.is_empty()
}

pub fn bracket_checker(barcket: &str) -> bool{
    let char_list: Vec<char> = barcket.chars().collect(); 

    let mut index = 0; 
    let mut balance = true; 
    let mut stack = Stack::new(); 

    while index < char_list.len() && balance{
        let ch = char_list[index]; 

        if ch == '(' {
            stack.push(ch);
        }else {
            if stack.is_empty(){
                balance = false
            }else{
                stack.pop(); 
            }
        }

        index += 1; 
    }
    
    balance && stack.is_empty()
}
