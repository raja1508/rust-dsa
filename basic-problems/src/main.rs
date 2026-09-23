use std::io;

fn main(){
    // char_to_ascii();
    // ascii_to_char();
    // char_type();
    // convert_to_lowercase();
    // are_brothers();
    // lexicographical_order();
    // replace_character();
    // remove_character();
    // trim_spaces();
    // toggle_case();
    // strong_password();
    // reverse();
    // palindrome();
    // count_words();

}


fn count_words(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let sentence : String = input.trim().parse().unwrap(); 

    let words: Vec<&str> = sentence.split_whitespace().into_iter().collect(); 
    println!("{}", words.len())

}

fn palindrome(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let word: String = input.trim().parse().unwrap(); 
    
    let char_array: Vec<char> = word.chars().collect();
    let mut ans: Vec<char>  = Vec::default(); 

    for i in (0..char_array.len()).rev() {
        let ch = char_array[i];  
        ans.push(ch);

    }

    if ans.iter().collect::<String>() == char_array.iter().collect::<String>(){
        print!("YES")
    }else {
        print!("NO")
    }

}

fn reverse(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let word: String = input.trim().parse().unwrap(); 
    
    let mut char_array: Vec<char> = word.chars().collect();
    let mut ans: Vec<char>  = Vec::default(); 

    for _ in 0..char_array.len() {
        if let Some(ch) = char_array.pop(){
            ans.push(ch);
        }; 
    }
    print!("{}", ans.iter().collect::<String>()); 

}

fn strong_password(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let word: String = input.trim().parse().unwrap(); 
    
    let char_array: Vec<char> = word.chars().collect();

    let mut password_parameters = vec![0; 4]; 

    let mut is_strong = true; 

    if char_array.len() != 10 {
        is_strong = false; 
    }

    for i in 0..char_array.len(){
        if char_array[i].is_ascii_lowercase() {
            password_parameters[0] += 1; 
        }else if char_array[i].is_ascii_uppercase() {
            password_parameters[1] += 1; 
        }else if char_array[i].is_ascii_digit(){
            password_parameters[2] += 1; 
        }else{
            password_parameters[3] += 1; 
        }

    }

    for k in 0..password_parameters.len(){
        if password_parameters[k] == 0 {
            is_strong = false; 
        }
    }

    if is_strong {
        print!("Strong")
    }else {
        print!("Weak")
    }

}

fn toggle_case(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let word: String = input.trim().parse().unwrap(); 
    
    let char_array: Vec<char> = word.chars().collect();
    let mut ans: Vec<char>  = Vec::default(); 

    for i in 0..char_array.len() {
        if char_array[i].is_ascii_lowercase(){
            ans.push(char_array[i].to_ascii_uppercase());
        }else{
            ans.push(char_array[i].to_ascii_lowercase());
        }
    }
    print!("{}", ans.iter().collect::<String>()); 

}

fn trim_spaces(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let word: String = input.trim().parse().unwrap(); 
    
    let char_array: Vec<char> = word.chars().collect();
    let mut ans: Vec<char>  = Vec::default(); 

    for i in 0..char_array.len() {
        if ' ' == char_array[i]{
            continue;
        }else{
            ans.push(char_array[i]);
        }
    }
    print!("{}", ans.iter().collect::<String>()); 

}

fn remove_character(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let word: String = input.trim().parse().unwrap(); 
    
    let char_array: Vec<char> = word.chars().collect(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let ch: char = input.trim().parse().unwrap(); 
    
    let mut ans: Vec<char>  = Vec::default(); 

    for i in 0..char_array.len() {
        if ch == char_array[i]{
            continue;
        }else{
            ans.push(char_array[i]);
        }
    }
    print!("{}", ans.iter().collect::<String>()); 

}

fn replace_character(){
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let word: String = input.trim().parse().unwrap(); 
    
    let mut char_array: Vec<char> = word.chars().collect(); 

    let mut input = String::new(); 
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let char1: char = parts.next().unwrap().parse().unwrap();
    let char2: char = parts.next().unwrap().parse().unwrap();

    for i in 0..char_array.len() {
        if char1 == char_array[i]{
            char_array[i] = char2; 
        }
    }
    print!("{}", char_array.iter().collect::<String>()); 

}

fn lexicographical_order(){
    let mut words = vec![String::default(); 2];

    for i in 0..2{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let word: &str = parts.next().unwrap();
        words[i] = word.to_string(); 
    }

    let  word_1: Vec<char> = words[0].chars().collect(); 
    let  word_2: Vec<char> = words[1].chars().collect();

    let  (mut ans , len) = if word_1.len() < word_2.len() {
        ("A".to_string(), word_1.len())
    }else if word_2.len() < word_1.len(){
        ("B".to_string(), word_2.len())
    }else {
        ("Equal".to_string(), word_1.len())
    };

    for i in 0..len {
        if word_1[i] > word_2[i]{
            ans = "B".to_string(); 
            break; 
        }else if word_1[i] < word_2[i]{
            ans = "A".to_string(); 
            break;
        }else {
            continue;
        }
    }
    print!("{}", ans); 

}

fn are_brothers(){
    let mut last_names = vec![String::default(); 2];

    for i in 0..2{
        let mut input = String::new(); 
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let _ = parts.next().unwrap(); 
        let last_name = parts.next().unwrap();
        last_names[i] = last_name.to_string(); 

    }

    if last_names[0] == last_names[1]{
        print!("YES")
    }else {
        print!("NO")
    }
     
}

fn convert_to_lowercase(){
    let mut input = String::new(); 

    io::stdin().read_line(&mut input).unwrap(); 

    let ch : char = input.trim().parse().unwrap(); 

    if ch.is_ascii_uppercase(){
        let uc = ch.to_ascii_lowercase(); 
        print!("{}", uc); 
        return; 
    }

    print!("{}", ch); 
}

fn char_type(){
    let mut input = String::new(); 

    io::stdin().read_line(&mut input).unwrap(); 

    let ch : char = input.trim().parse().unwrap(); 

    if ch.is_ascii_lowercase(){
        print!("Lowercase"); 
        return; 
    }

    if ch.is_ascii_uppercase(){
        print!("Uppercase"); 
        return; 
    }

    if ch.is_ascii_digit(){
        print!("Digit");
        return
    }

    println!("Special"); 
}

fn ascii_to_char(){
    let mut input = String::new(); 

    io::stdin().read_line(&mut input).unwrap(); 

    let ascii : u8 = input.trim().parse().unwrap(); 

    println!("{}", ascii as char); 
}

fn char_to_ascii(){
    let mut input = String::new(); 

    io::stdin().read_line(&mut input).unwrap(); 

    let ch : char = input.trim().parse().unwrap(); 

    println!("{}", ch as u8); 
}