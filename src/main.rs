use proconio::input;

fn main() {
    input! {
        n: usize
    }
    let ans = func(n);
    println!("{}", ans)
}

fn func(k: usize) -> usize {
    if k == 0 {
        1
    } else {
        k * func(k - 1)
    }
}
