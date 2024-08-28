// 可以建立与服务器连接
// 客户端可以通过stream发送数据，读取服务器传入的数据

use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    // 三次握手 1.发出建立连接请求，SYN包
    // 三次握手 3.确认接收到SYN-ACK包，返回ACK包
    let mut stream = TcpStream::connect("localhost:3000").unwrap();

    // 向服务器发送数据，先发送到服务器的缓冲区
    stream.write("Hello".as_bytes()).unwrap();
    stream.write("???".as_bytes()).unwrap();
    println!("Data sent");

    // // 初始化buffer
    // let mut buffer = [0; 10];
    // // 从服务器读取数据写入buffer
    // stream.read(&mut buffer).unwrap();
    //
    // println!(
    //     "Response from server:{:?}",
    //     str::from_utf8(&buffer).unwrap()
    // );
}
