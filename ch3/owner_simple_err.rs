// 問題にあるプログラム
fn main() {
    let g1 = String::from("穏やかな心は体に良い");
    let g2 = g1; // 所有権をg2に移動
    println!("{}", g1); // 空っぽのg1を利用できる？
}

/*
$ rustc owner_simple_err.rs && ./owner_simple_err
warning: unused variable: `g2`
 --> owner_simple_err.rs:4:9
  |
4 |     let g2 = g1; // 所有権をg2に移動
  |         ^^ help: if this is intentional, prefix it with an underscore: `_g2`
  |
  = note: `#[warn(unused_variables)]` on by default

error[E0382]: borrow of moved value: `g1`
 --> owner_simple_err.rs:5:20
  |
3 |     let g1 = String::from("穏やかな心は体に良い");
  |         -- move occurs because `g1` has type `String`, which does not implement the `Copy` trait
4 |     let g2 = g1; // 所有権をg2に移動
  |              -- value moved here
5 |     println!("{}", g1); // 空っぽのg1を利用できる？
  |                    ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0382`.
*/
