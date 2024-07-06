// Fix the error with the use of define_x
fn main() {
    let x: String = define_x();
        println!("{}, world", x); 
    }
    
    fn define_x() {
        let x = "hello";
        return x;
    }
    