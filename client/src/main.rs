use std::net::TcpStream;
use std::io::{self,prelude::*,BufReader,Write};
use std::str;
fn main()->io::Result<()> {

    let mut stream = TcpStream::connect("127.0.0.1:8081")?; //连接8081端口
    loop {
        let mut input = String::new();  //读取输入字符
        io::stdin().read_line(&mut input).expect("读取失败"); //从控制台获取用户输入
        stream.write(input.as_bytes()).expect("写入失败");   //用户输入数据写入链接中
        let mut reader = BufReader::new(&stream); 
        let mut buffer:Vec<u8> = Vec::new();
        //一直读到换行
        reader.read_until(b'\n',&mut buffer).expect("读取失败");
        println!("read from server :{}",str::from_utf8(&mut buffer).unwrap());
    }
    Ok(())
}
