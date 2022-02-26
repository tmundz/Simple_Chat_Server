use tokio::{net::TcpListener, 
    io::{AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt} 

};


#[tokio::main]
async fn main() {
    //incremental step tpc echo server
    
    //tcp  listener .await will suspend current task until needed
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();  
    

   

    loop {
        let ( mut socket, _addr) = listener.accept().await.unwrap();
        tokio::spawn (async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();
        
    
            loop {
                    
                    let bytes_read = reader.read_line(&mut line).await.unwrap();
                    if bytes_read == 0 {
                        break; 
                    }
                    //send back 
                    writer.write_all(line.as_bytes()).await.unwrap(); // writes back to the client
                    line.clear();
            }
    });

    }
}
