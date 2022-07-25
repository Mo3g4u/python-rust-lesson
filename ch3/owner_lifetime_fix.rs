fn gen_message() -> String {
    let msg = String::from("過ちを見過ごす人は美しい");
    return msg;
}

fn main() {
    // 実体が返ってくるので所有権がmoveする
    let m = gen_message();
    println!("{}", m);
}
