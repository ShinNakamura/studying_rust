/// `&str`は
/// * 任意のスライスでも
/// * プログラム内に直に書かれたリテラルでも
/// * (実行時に確保され、解放される)`String`でも
/// 指すことができる。
/// したがって、関数の引数として両方の種類の文字列を渡したい場合
/// 仮引数の型は`&str`とするのが妥当だ
/// > プログラミングRUST P.66

/// cargo run --bin str_string
fn main(){
    fn say(s: &str) -> (){
        println!("{}", s);
    }

    let foo = "foo";
    say(foo); // 任意のスライス
    say("bar"); // プログラム内に直に書かれたリテラル
    say(&foo.to_string()); // `String`への参照
}


