fn main() {
    // Rust は UTF-8に対応している
    // 日本語は3バイト、半角英数は1バイトで定義される。
    let s1 = "helloこんにちは挨拶";
    let s2 = "hello";

    println!("Stack address of s1 is:{:p}", &s1);
    println!("Stack address of s2 is:{:p}", &s2);

    println!("Stack memory address of s1 is:{:?}", s1.as_ptr());
    println!("Stack memory address of s2 is:{:?}", s2.as_ptr());

    println!("Len of s1 is:{:?}", s1.len());
    println!("Len of s2 is:{:?}", s2.len());

    // String型を使ってみる。
    let mut s1 = String::from("Hello");
    let mut s2: String = String::from("hello_world");
    println!("Stack address of s1 is:{:p}", &s1);
    println!("Stack address of s2 is:{:p}", &s2);
    println!("Heap memory address of s1 is:{:?}", s1.as_ptr());
    println!("Heap memory address of s2 is:{:?}", s2.as_ptr());
    println!("Len of s1 is:{:?}", s1.len());
    println!("Len of s2 is:{:?}", s2.len());
    println!("Capacity of s1 is: {}", s1.capacity());
    println!("Capacity of s2 is: {}", s2.capacity());

    // 新しい文字列を変数の末尾に追加する
    s1.push_str("_new1");
    s2.push_str("_new2");
    println!("{} {}", s1, s2);
}
