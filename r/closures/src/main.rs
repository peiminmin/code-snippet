use std::thread;

fn main() {
    // 闭包不可变借用外部上文变量
    let mut list1 = vec![1, 2, 3, 4];

    //    let mut mut_borrow = || list1.push(5); 如果写在这里，发生了可变借用于不可变借用作用域存在交叉： 函数定义属于定义，函数运行属于变量使用
    let only_borrow = || println!("only borrow print list[0]:{}", list1[0]);
    only_borrow();

    // 闭包可变借用外部上文变量
    let mut mut_borrow = || list1.push(5);
    mut_borrow();

    // only_borrow(); 如果写在这里，如果写在这里，发生了可变借用于不可变借用作用域存在交叉

    //线程运行
    thread_print();

    //线程启动如果不join到主线程，会导致子线程成为孤儿线程
    thread_print2();
}

fn thread_print() {
    let list = vec![1, 2, 3];
    thread::spawn(move || println!("{:?}", list))
        .join()
        .unwrap();
}

fn thread_print2() {
    let handler1 = thread::spawn(|| {
        for i in 1..100 {
            println!("thread 1 print:{}", i)
        }
    });

    let handler2 = thread::spawn(|| {
        for i in 1..100 {
            println!("thread 2 print: {}", i);
        }
    });

    // handler.join().unwrap();不join到主线程，会导致子线程成为孤儿线程

    for i in 1..100 {
        println!("main thread print:{:?}", i)
    }
    handler1.join().unwrap(); //不join到主线程，会导致子线程成为孤儿线程
    handler2.join().unwrap();
}

fn thread_print3() {
    let list = vec![1, 2, 3, 4];

    let handler = thread::spawn(move || println!("{:?}", list)); //多线程不能引用外部的资源

    handler.join().unwrap();
}
