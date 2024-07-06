// mod print;
// mod vars;
// mod types;
mod algo;
fn main() {
   
    // print::run();
    // types::run();
    let nums: [i32; 5]  = [2,4,6,8,10];
    algo::sumnumbers(nums);
  let s =  algo::reversestring("Hello");
    println!("{}", s);
}
