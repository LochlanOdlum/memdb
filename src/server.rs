use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Error, ErrorKind};
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;

#[derive(Debug, Eq, Hash, PartialEq)]
enum Command {
    Ping,
    Echo,
    Set,
    Get,
    Del,
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PING" => Ok(Command::Ping),
            "ECHO" => Ok(Command::Echo),
            "SET" => Ok(Command::Set),
            "GET" => Ok(Command::Get),
            "DEL" => Ok(Command::Del),
            _ => Err(format!("Fail")),
        }
    }
}

#[derive(Debug)]
struct MemoryStore {
    store: HashMap<Command, String>,
}

impl MemoryStore {
    fn handle(&mut self, command: Command, val: &str) -> Result<(), String> {
        self.store.insert(command, String::from(val));

        Ok(())
    }

    fn new() -> Self {
        Self {
            store: HashMap::<Command, String>::new(),
        }
    }
}

pub fn run() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;

    let mut memory_store: MemoryStore = MemoryStore::new();

    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream, &mut memory_store)?;
    }

    Ok(())
}

fn handle_connection(stream: TcpStream, mem_store: &mut MemoryStore) -> io::Result<()> {
    println!("Handling connecton");
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        let line = line?;
        let split_lines = line
            .split(" ")
            .map(|s| String::from(s))
            .collect::<Vec<String>>();

        if split_lines.len() < 2 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Must provide an argument parameter",
            ));
        }

        let command = Command::from_str(split_lines.get(0).unwrap())
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid command"))?;

        mem_store.handle(command, split_lines.get(1).unwrap());

        println!("{}", line);
        println!("{:#?}", mem_store);
    }
    Ok(())
}
