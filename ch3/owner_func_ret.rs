fn main() {
    let mut g1 = String::from("過ちを見過ごす人は美しい");
    g1 = show_message(g1);
    println!("{}", g1);
}

// Stringを受け取り、Stringを返す関数
fn show_message(message: String) -> String {
    println!("{}", message);
    return message;
}
