pub fn run() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let v_iter = v.iter();
    for i in v_iter {
        println!("{}", i);
    }
    let v2: Vec<_> = v
        .iter()
        .map(|x| x + 1)
        .collect();

    assert_eq!(v2, vec![2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let shoes = vec![
        Shoes {
            size: 42,
            style: String::from("J4"),
        },
        Shoes {
            size: 42,
            style: String::from("puma"),
        },
        Shoes {
            size: 41,
            style: String::from("Nike"),
        },
        Shoes {
            size: 41,
            style: String::from("Nike"),
        }
    ];
    let shoes_in_my_size = shoes_my_size(shoes, 41);
    println!("Shoes in my size are:{}", shoes_in_my_size.len());
}

#[derive(PartialEq, Debug)]
struct Shoes {
    size: u32,
    style: String,
}

fn shoes_my_size(shoes: Vec<Shoes>, size: u32) -> Vec<Shoes> {
    shoes
        .into_iter()
        .filter(|s| s.size == size)
        .collect()
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]

fn call_counter_directly(){
    let mut counter : Counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}