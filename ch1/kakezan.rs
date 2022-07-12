// 掛け算をするだけの関数を定義
fn kakezan(a: i64, b: i64) -> i64 {
    // Rustで関数の値を戻すには、関数の末尾で「値」を書くか「return 値;」とする
    a * b // もしくは return a * b;
}

fn main() {
    // 関数を呼び出す
    let ex1 = kakezan(3, 5);
    println!("3*5={}", ex1);
    let ex2 = kakezan(8, 4);
    println!("8*4={}", ex2);
}
