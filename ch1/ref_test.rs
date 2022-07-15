fn main() {
    // 変数vを10にする
    let mut v = 10;
    // 関数を呼び出す
    set_value(&mut v);
    // 変数の値は？
    println!("v={}", v);
}

// 引数の値を100に変更する関数
fn set_value(arg: &mut u32) {
    // 書き換えるときは参照を実体に元してから値を書き換える必要がある＝デリファレンス
    *arg = 100;
}
