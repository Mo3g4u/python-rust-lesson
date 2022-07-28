// 構造体の差分コピーについて
struct Person {
    name: String,
    age: i32,
}
impl Person {
    fn new(name: &str, age: i32) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    // Taroを作成
    let taro = Person::new("Taro", 18);
    // JiroはTaroを複製して名前だけ変えたい
    let jiro = Person {
        name: String::from("Jiro"),
        ..taro // 構造体更新記法　この場合はname以外のフィールドをコピーする
    };

    // TaroとJiroを表示
    println!("{},{}", taro.name, taro.age);
    println!("{},{}", jiro.name, jiro.age);
}

/*
構造体を複製する他の方法としては、構造体にCopyトレイトを実装して明示的に構造体を
複製する方法があります。また、構造体宣言の直前に「#[derive (Clone)]」と記述すると、
自動的にCloneトレイトを実装し構造体を複製できるようになります。
*/
