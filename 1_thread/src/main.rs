use std::sync::mpsc;
use std::thread;

/**
在这个示例中，第一个线程简单地打印了一条消息并退出。第二个线程通过通道发送了一条消息给主线程，主线程接收并打印了这条消息。
*/
fn main() {
    // 创建一个新的线程
    let handle = thread::spawn(|| {
        println!("Hello from the new thread!");
    });

    // 等待新线程完成
    handle.join().unwrap();

    // 使用通道在线程间通信
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let msg = "Hello from the communication thread!";
        tx.send(msg).unwrap();
    });

    // 接收来自线程的消息
    let received = rx.recv().unwrap();
    println!("{}", received);
}
