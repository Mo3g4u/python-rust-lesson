fn main() {
    let pr = "猫に小判";
    // 1バイトずつ表示
    for c in pr.bytes() {
        print!("{:2x} ", c);
    }
    // バイトを得る &strのlenメソッドは文字数ではなくてバイト数を返す
    println!("\nバイト数={}B", pr.len());
}
