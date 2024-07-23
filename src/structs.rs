struct User {
    name: String,
    email: String,
    sign_in_account: u64,
    active: bool,
}
#[derive(Debug)]
struct Dimensions {
    l: u32,
    w: u32,
}
impl Dimensions {
    fn area(&self) -> u32 {
        self.l * self.w
    }
    fn can_hold(&self, other: &Dimensions) -> bool {
        self.l > other.l && self.w > other.w
    }
}

impl Dimensions {
    fn square(size: u32) -> Dimensions {
        Dimensions { l: size, w: size }
    }
}

pub fn run() {
    let dimens = Dimensions { l: 20, w: 30 };
    let rec1 = Dimensions { l: 50, w: 40 };
    let rec2 = Dimensions { l: 40, w: 30 };
    let rec3 = Dimensions::square(30);
    println!("{}", rec2.can_hold(&rec1));
    let a = dimens.area();
    print!("The area is:{}\n", a);
    print!("The Dimension:{:#?}", dimens);
    let mut user1 = User {
        name: String::from("Nash"),
        email: String::from("kim@gmail.com"),
        sign_in_account: 10,
        active: true,
    };

    let username = user1.name;

    user1.name = String::from("Kim");
    let user2 = build_user(String::from("Nash"), String::from("Nash"));
    let user3 = User {
        name: String::from("Nash"),
        email: String::from("kim@gmail.com"),
        ..user2
    };
    struct Point(i32, i32, i32);
}

fn build_user(name: String, email: String) -> User {
    User {
        name: name,
        email: email,
        sign_in_account: 11,
        active: true,
    }
}
