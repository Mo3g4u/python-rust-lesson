// Rustでサイコロを振るプログラム
// randクレートのRngを利用 --- (*1)
use rand::Rng;

fn main() {
    // 乱数の生成器を用意 --- (*2)
    let mut rng = rand::thread_rng();
    // 5回サイコロを振る
    for _ in 0..5 {
        // 1から6の乱数を生成 --- (*3)
        let dice = rng.gen_range(1..=6);
        println!("{}", dice);
    }
}

/*
1..10  range(1, 10) 1から9までの範囲を表す
1..=10 range(1, 11) 1から10までの範囲を表す
0..10  range(10)    0から9までの範囲を表す
*/
