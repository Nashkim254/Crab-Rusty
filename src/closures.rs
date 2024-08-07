use std::thread;
use std::time::Duration;

pub fn run() {
    let intensity = 10;
    let random = 7;
    let x = 4;
    let equal_to_x = |z| x == z;
    let y = 4;
    assert!(equal_to_x(y));

    generate_workout(intensity, random);
}
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating  slowly....");
    thread::sleep(Duration::from_secs(2));
    intensity
}
struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
fn generate_workout(intensity: u32, random_number: u32) {
    let mut cache_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today do , {} pushups", cache_result.value(intensity));
        println!("Next do , {} sit ups", cache_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today and remember to get hydrated...");
        } else {
            println!("Today run for , {} minutes", cache_result.value(intensity));
        }
    }
}
