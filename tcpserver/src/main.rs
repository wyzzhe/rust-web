// 持续监听某端口，可以监听到连接的建立
// 可以通过stream读取客户端传入的数据，可以发送数据给客户端

use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // listener绑定到本地端口3000
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000 ....");

    loop {}
    // stream是字节流(可能包含SYN、SYN-ACK、ACK、客户端传入数据、服务器传出数据)
    // 三次握手 2.接收建立连接请求，返回SYN-ACK包
    // listener.incoming() 提供了一个迭代器，用于监听连接请求。
    // 这个迭代器会阻塞，直到有新的连接请求到达。如果没有连接请求，它会一直等待。
    // 当有新的连接请求到达时，迭代器会返回一个 Result<TcpStream, std::io::Error> 类型的值
    // TcpStream { addr: 127.0.0.1:3000, peer: 127.0.0.1:49789, fd: 4 }
    // for stream in listener.incoming() {
    //     let mut stream = stream.unwrap();
    //     println!("Connection established");
    //     println!("Client: {:?}", stream);
    // }
}
//
//     let mut buffer = [0; 1024];
//     // 从客户端读取数据写入buffer
//     stream.read(&mut buffer).unwrap();
//     // 从buffer读取数据传递给客户端
//     stream.write(&mut buffer).unwrap();