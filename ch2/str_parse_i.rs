fn main() {
    // 文字列に数値を代入
    let s = "365";
    // i32型の数値に変換
    let i: i32 = match s.parse() {
        Ok(v) => v,  // 成功した時
        Err(_) => 0, // 失敗した時
    };
    println!("{}", i);
}
