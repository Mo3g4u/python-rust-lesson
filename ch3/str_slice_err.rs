// 問題のあるプログラム
// もしスライスで文字の途中を指定したら
fn main() {
    let pr = "知識は武器よりも価値がある。";
    println!("{}", &pr[1..6]); // 文字の途中を取り出す
}