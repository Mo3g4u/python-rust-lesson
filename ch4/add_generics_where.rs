// ジェネリクス関数でwhereを使う
fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

// 関数を使ってみる
fn main() {
    println!("{}", add(10, 25));
    println!("{}", add(10.0, 25.0));
}

/*
長いトレイトの宣言はwhereを使って見やすく宣言できる
*/
