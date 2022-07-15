// BigIntを使う為の宣言
// use を使うことで指定したクレートのモジュールを任意のスコープに取り込むことができる
use num_bigint::BigInt;

fn main() {
    // BigIntのオブジェクトを作って値を設定
    let v = BigInt::from(1234);
    // 5678乗を計算
    println!("{}", v.pow(5678));
}
