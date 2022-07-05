use std::net::{TcpListener,TcpStream}; //导入std tcp相关依赖包
use std::thread;                        //导入线程相关依赖包
use std::io::{self,Read,Write};         //导入io相关依赖包

fn handle_client(mut stream:TcpStream)->io::Result<()>{
    let mut buf = [0;512];  //读取buf字节
    loop{
        let bytes_read = stream.read(&mut buf)?; //读取到的字节数
        println!("bytes_read：{}",bytes_read);
        if bytes_read == 0{
           return Ok(());     
        }
        stream.write(&buf[..bytes_read]);
    }
}

fn main()->io::Result<()> {
    let listenr = TcpListener::bind("127.0.0.1:8081")?;      //绑定本地8081端口
    let mut thread_vec:Vec<thread::JoinHandle<()>> = Vec::new();  //创建一个线程池
    //通过for 循环监听连接
    for stream in listenr.incoming(){
        let stream = stream.expect("fail");
        //把连接过来的请求放入线程异步处理
        let handle = thread::spawn(move || {
            //连接处理
            handle_client(stream).unwrap_or_else(|err|eprintln!("{:?}",err));
        });
        //把线程放到 vec
        thread_vec.push(handle);
    }
    //等待每个线程都结束
    for handle in thread_vec{
        handle.join().unwrap();
    }
    Ok(())
}
