use std::vec;

pub fn run() {
    // Vector 型はヒープ領域を消費する
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let _v3 = vec![9, 10];

    println!("Stack address of v1 is: {:p}", &v1);
    println!("Stack address of v2 is: {:p}", &v2);

    // Stack address of v1 is: 0x16bbde9a8 => a8 => 168
    // Stack address of v2 is: 0x16bbde9c0 => c0 => 192
    // 192 - 168 = 24
    // 24バイト消費されていることから実はVector型はString型と同じデータ構造を持つ
    // Vector型は要素毎にバイト数をもつ。

    // ヒープメモリ内にある実データの先頭アドレスを表示する
    println!("Heap Memory Address of v1: {:p}", v1.as_ptr());

    // この時表示されるLen は要素の数
    println!("Len of v1 is {}", v1.len());
    println!("Capacity of v1 is {}", v1.capacity());

    // Vector 型ではインサートメソッドが使える
    println!("{:?}", v1);
    v1.insert(1, 10);
    println!("{:?}", v1);

    // 要素を削除する
    v1.remove(1);
    println!("{:?}", v1);

    // v3をv1に連結する
}
