pub fn run() {
    let name = "Nash";
    let mut age = 26;
    age = 27;
    const PI: f32 = 3.142;
    let (my_name, my_age) = ("Kim", 26); //tuples
    println!("My name is {} and i am {}", my_name, my_age);
    println!("Pi is {} ", PI);
    let mut counter = 0;

    while counter < 10{
        counter += 1;
        println!("counter is {}", counter);
    }
    let num = loop {
        if counter == 10 {
            break counter;
        }
        counter += 1;
        println!("counter is {}", counter);
    };
    println!("final counter is {}", counter);
}
