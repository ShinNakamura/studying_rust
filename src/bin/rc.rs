/// 所有権の共有。参照カウンタ
/// > プログラミングRUST P.87
/// cargo run --bin rc

use std::rc::Rc;

fn main() {
    // `Rc<T>`の値は、ヒープ上に確保したTとそれに付随する参照カウンタを指すポインタ
    let s: Rc<String> = Rc::new("foo".to_string());

    // `Rc<T>`を`clone`してもヒープ上のT値はクローンされず、
    // 同じものを指すポインタが作られ参照カウンタがインクリメントされる
    let t: Rc<String> = s.clone();
    let u: Rc<String> = s.clone();
    // Rc型のポインタ自体には通常の所有権ルールが適用される

    // `String`の普通のメソッドは`Rc<String>`にも直接実行可能
    assert!(s.contains("oo"));
    assert_eq!(t.find("f"), Some(0));
    println!("u={}", u);

    // Rcポインタに所有される値は不変となる。
}
