#[derive(Debug)]

// 構造体でジェネリクスを指定する
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let pt_i = Point { x: 20, y: 50 };
    let pt_f = Point { x: 20.5, y: 15.3 };
    println!("{:?}", pt_i);
    println!("{:?}", pt_f);
}

/*
[書式] 構造体でジェネリクスを指定
struct 構造体名 <T, U> {
    フィールド1: T,
    フィールと2: U,
}
*/
