fn main() {
    let mut total = 0;
    // fot 変数 in イテレータ
    for i in 1..11 {
        total += i;
    }
    println!("{}", total);
}
