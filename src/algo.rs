pub fn sumnumbers(arr: [i32; 5]) {
    let mut sum = 0;
    for i in arr.iter() {
        sum += i;
    }
    println!("{}", sum);
}

pub fn reversestring(s: &str) -> String {
    let res = s.split("").collect::<Vec<_>>();
    let mut i = res.len() - 1;
    let mut reversed: String = "".to_string();
    while i > 0 {
        reversed = reversed + res[i];

        i -= 1;
    }
    return reversed;
}
