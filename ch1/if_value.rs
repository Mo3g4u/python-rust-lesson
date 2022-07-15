fn main() {
    let n = 5;
    // 参考演算子に似た機能 if が値を返す
    let hantei = if n % 2 == 0 { "偶数" } else { "奇数" };
    println!("{}", hantei);
}
