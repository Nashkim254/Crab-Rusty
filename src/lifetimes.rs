
struct ImportantExcerpt <'a>{
part: &'a str,
}




pub fn run(){
 let s: &'static str = "I have a static lifetime";
let  s1 = String::from("Hello");
let  s2 = String::from("World!");
let longest = longest_string(s1.as_str(), s2.as_str());
    println!("Longest string {}", longest);
}

//lifetime parameters/anotations
fn longest_string<'a>(s1: &'a str, s2: &'a str) -> &'a str {

    if s1.len() > s2.len() {
        return s1;
    }else{
        return s2;
    }
}