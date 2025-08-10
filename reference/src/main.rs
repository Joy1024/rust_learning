/*
为避免频繁所有权转移，Rust 引入借用（Borrowing）机制。
1、引用就是所有权借用的一种方式。
2、借用允许函数使用变量的引用，而不是所有权转移。
Rust 的借用检查基于三个核心规则，同一作用域：
1. 任意时刻，只能有一个可变引用，或多个不可变引用
2. 引用必须总是有效的
3. 不能同时存在可变和不可变引用
 */

fn change(s: &mut String) {
    s.push_str("=>changed");
}

fn main() {
    let mut s = String::from("change"); //s为change所有权
    println!("所有权s: {}", s);
    let ref0 = &s;      //一个不可变引用
    println!("不可变ref0: {}", ref0);

    let ref1 = &s;      //第二个不可变引用
    println!("不可变ref1: {}", ref1);
    
    // let ref2 = &mut s;  //编译错误：不能同时有可变和不可变引用
    // let ref3 = &mut s; //只能一个可变引用
    // let ref3 = &s; //第二个不可变引用

    println!("第1次修改");
    change(&mut s); //传递可变引用
    println!("所有权: {}", s);

    println!("第2次修改");

    change(&mut s); //传递可变引用
    println!("所有权: {}", s);

 

}
