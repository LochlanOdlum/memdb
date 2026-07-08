
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Error};
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;

enum Command {
  PING,
  ECHO,
  SET,
  GET,
  DEL,
}

impl FromStr for Command {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "GET" => Ok(Command::GET),
      _ => Err(format!("Fail"))
    }
  }
}

pub fn run() -> io::Result<()> {
  let listener = TcpListener::bind("127.0.0.1:6379")?;


  let mut memory_store: HashMap<String, String> = HashMap::new();

  for stream in listener.incoming() {
      let stream = stream?;
      handle_connection(stream, &mut memory_store)?;
  }

  Ok(())
}


fn handle_connection(stream: TcpStream, mem_store: &mut HashMap<String, String>) -> io::Result<()> {
    println!("Handling connecton");
    let reader = BufReader::new(stream);

    let mut invalid_line = false;

    for line in reader.lines() {
      let line = line?;
      let split_lines = line
        .split(" ")
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

      if split_lines.len() >= 2 {
        invalid_line = true;
      }

      mem_store.insert(
        String::from(split_lines.get(0).unwrap()), 
        String::from(split_lines.get(1).unwrap())
      );


      println!("{}", line);
      println!("{:#?}", mem_store);
    }
    Ok(())

    // if invalid_line {
    //   Err(Error::new(
    //     io::ErrorKind::InvalidData,
    //     "invalid line",
    // ))
    // } else {
    //   Ok(())
    // }
}