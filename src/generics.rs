pub fn run() {
    let number_list: Vec<i32> = vec![10, 34, 28, 54, 67, 29, 40];

    let largest = find_largest(number_list);
    println!("The largest no. is {}", largest);
    let p1 = Point { x: 10, y: 7 };
    let p2 = Point { x: 5, y: 10.3 };
}

fn find_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for i in number_list {
        if i > largest {
            largest = i;
        }
    }

    largest
}
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
