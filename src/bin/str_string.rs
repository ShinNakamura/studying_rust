/// cargo run --bin str_string
fn main(){
    /// `&str`は
    /// * 任意のスライスでも
    /// * プログラム内に直に書かれたリテラルでも
    /// * (実行時に確保され、解放される)`String`でも
    /// 指すことができる。
    /// したがって、関数の引数として両方の種類の文字列を渡したい場合
    /// 仮引数の型は`&str`とするのが妥当だ
    /// > プログラミングRUST P.66
    fn say(s: &str) -> (){
        println!("{}", s);
    }

    let foo = "foo";
    say(foo); // 任意のスライス
    say("bar"); // プログラム内に直に書かれたリテラル
    say(&foo.to_string()); // `String`への参照


    // `String`ではなく`&str`で`name`属性を定義する
    #[derive(Debug)]
    struct Person<'a> { // `Person`構造体と`name`属性の値(`&str`)は生存期間が同じであることを明示する
        name: &'a str,
        age: u32,
    }
    // さすれば、`name:"max"`と書けば済む
    // `name:"max".to_string()`とした場合のような余計なアロケーションは発生しない
    let driver = Person { name:"max", age:32, };
    println!("{:#?}", driver);
    println!("{}", driver.name); // => max

    // 下記の構造体はコンパイルエラーになる
    // `&str`の生存期間が不明だから
    // struct P {
    //     name: &str,
    //     age: u32,
    // }
}


