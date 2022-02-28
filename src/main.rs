use tokio::{net::TcpListener, 
    io::{AsyncWriteExt, BufReader, AsyncBufReadExt}, sync::broadcast 

};
use colored::*;


#[tokio::main]
async fn main() {
    //incremental step tpc echo server
    
    //tcp  listener .await will suspend current task until needed
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();  

    //this will allow users to send messages to other users 
    //specifically tells the compiler they are sending strings
    let (txt, _rec) = broadcast::channel(10);
    

   
    //this loop will wait till someone connects to the server 
    loop {
        let txt = txt.clone(); //moves txt to the new task 
        let mut rec = txt.subscribe(); //pulls the reciever from the sender
        let ( mut socket, addr) = listener.accept().await.unwrap();

        //this will create a new task with tokio spawn
        tokio::spawn (async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();
        
            //this loop read the bytes that a user sends 
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                    if result.unwrap() == 0 {
                       break;
                    }
                    txt.send((line.clone(), addr)).unwrap();
                    
                    line.clear();
                }
                    result = rec.recv() => {

                        let (msg, other_addr) = result.unwrap();
                      
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                    
                }
            }   
        });
    }
}
