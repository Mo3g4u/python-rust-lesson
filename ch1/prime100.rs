// Rustで素数を100個生成

// 素数判定をする関数
fn is_prime(n: usize) -> bool {
    // usize 符号無し整数 OSアーキテクチャに依存
    // このケースでは特にビット数を意識せずに「なんとなく使える」と理由でusize型が選ばれている
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

// 素数を100個求める関数
fn get_primes(primes: &mut [usize; 100]) {
    let mut i = 2;
    let mut count = 0;
    // countが100に達するまで繰り返す
    while count < 100 {
        if is_prime(i) {
            primes[count] = i;
            count += 1;
        }
        i += 1;
    }
}

fn main() {
    // 初期値の配列を100個用意する
    // 要素が100個ある配列変数を初期値0で初期化して、配列変数primesに代入している
    let mut primes = [0; 100];
    // 素数を100個求める
    // Rustでは可変参照な値を引数に指定して関数を呼び出す場合、呼び出し側でも可変であることを指定する必要がある
    get_primes(&mut primes);

    // 結果を表示
    println!("{:?}", primes);
}
