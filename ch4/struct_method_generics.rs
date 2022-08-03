#[derive(Debug)]

// 構造体でジェネリクスを指定する
struct Point<T> {
    x: T,
    y: T,
}

// メソッドを定義
// std::ops::AddAssignトレイトは足し算の代入を実装することを強制
impl<T> Point<T>
where
    T: std::ops::AddAssign,
{
    // コンストラクター
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    // 値を加算
    fn add(&mut self, pt: Point<T>) {
        self.x += pt.x;
        self.y += pt.y;
    }
}

fn main() {
    // Point型を生成
    let mut pt = Point::new(10, 10);
    println!("{:?}", pt);
    // 座標に値を加算
    pt.add(Point { x: 20, y: 30 });
    println!("{:?}", pt);
}
