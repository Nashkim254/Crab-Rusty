pub fn run() {
    //------Ownership rules
    //each rust value has a variable thats called  its owner
    //there can only be one owner at a time
    //when the owner goes out of scope the value is dropped

    let mut s = String::from("Hello!");
    take_ownership(s.clone());

    let l = calculate_length(&s);
    print!("The legth of {} is:{}\n", s, l);
     mutate_string(&mut s);
    print!("The value is:{}\n", s);
}

fn take_ownership(some_string: String) {
    print!("{}, world\n", some_string);
}
fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn mutate_string(s: &mut String) {
     s.push_str("World");
    
}
