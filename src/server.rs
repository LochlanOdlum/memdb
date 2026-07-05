
use std::io::{self, BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

pub fn run() -> io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:6379")?;

  for stream in listener.incoming() {
      let stream = stream?;
      handle_connection(stream)?;
  }

  Ok(())
}


fn handle_connection(stream: TcpStream) -> io::Result<()> {
    println!("Handling connecton");
    let reader = BufReader::new(stream);

    for line in reader.lines() {
      let line = line?;
      println!("{}", line);
    }
    Ok(())
}