use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let ans = if (a <= b && b <= c) || (c <= b && b <= a) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
