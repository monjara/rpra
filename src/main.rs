use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: String
    }
    let len = s.chars().count();
    let index = ((len + 1) / 2) - 1;
    println!("{}", s.chars().nth(index).unwrap())
}
