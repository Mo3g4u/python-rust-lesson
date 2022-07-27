fn main() {
    // 変数sに文章を代入
    let s = format!(
        "{}{}",
        "There is more happiness in giving", "than there is in receiving"
    );

    // クロージャで検索
    let res = s.find(|c: char| c.to_ascii_uppercase() == 'S');
    match res {
        Some(i) => println!("S={}B", i),
        None => println!("None"),
    };
}
