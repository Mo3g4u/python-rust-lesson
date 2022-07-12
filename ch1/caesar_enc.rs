// Rustでシーザー暗号に変換する関数

fn encrypt(text: &str, shift: i16) -> String {
    // 'A'と'Z'の文字コードをi16型で得る
    // as は強制的に型変換を行う機能
    let code_a = 'A' as i16;
    let code_z = 'Z' as i16;
    // 結果を代入する変数を用意
    let mut result = String::new();
    // 一文字ずつ繰り返す
    for ch in text.chars() {
        // 文字コードに変換
        let mut code = ch as i16;
        // AからZの間か？
        if code_a <= code && code <= code_z {
            // shift分だけずらす
            code = (code - code_a + shift + 26) % 26 + code_a;
        }
        // 文字コードから文字に変換
        // i16からcharへは変換できないので一度u8に変換してからcharに変換している
        result.push((code as u8) as char);
    }
    return result;
}

fn main() {
    // 関数を呼び出す
    let enc = encrypt("I LOVE YOU.", 3);
    let dec = encrypt(&enc, -3);
    println!("{} => {}", enc, dec)
}
