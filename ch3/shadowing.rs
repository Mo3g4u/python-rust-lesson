fn main() {
    // シャドーイングを使わない例
    {
        let mut v = 300; // vをミュータブルにする
        v = v + 5;
        println!("{}", v);
    }
    // シャドーイングを使う例
    {
        let v = 300; // vはイミュータブルでOK
        let v = v + 5;
        println!("{}", v);
    }
}
