use std::collections::HashMap;
use std::io::{self, BufRead, BufReader, Error, ErrorKind, Write};
use std::net::{TcpListener, TcpStream};
use std::str::FromStr;


#[derive(Debug, Eq, PartialEq)]
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
    store: HashMap<String, String>,
}

impl MemoryStore {
    fn handle(&mut self, command: &Command, args: &[String]) -> Result<Option<String>, String> {
        return match command {
            Command::Ping => {
                Ok(Some(format!("PONG")))
            }
            Command::Echo => {
              Ok(Some(args.join(" ")))
            }
            Command::Set => {
                if args.len() != 2 {
                    return Err(format!("Two arguments expected for this command"));
                }

                self.store.insert(
                  String::from(args.get(0).unwrap()),
                 String::from(args.get(1).unwrap()),
                );

                Ok(None)
            }
            Command::Get => {
                if args.len() != 1 {
                    return Err(format!("One argument expected for this command"));
                }

                let key = args.get(0).unwrap();

                Ok(self.store.get(key).cloned())
            }
            Command::Del => {
                if args.len() != 1 {
                    return Err(format!("One argument expected for this command"));
                }

                 let key = args.get(0).unwrap();

                self.store.remove(key);

                Ok(None)
            }
        }

    }

    fn new() -> Self {
        Self {
            store: HashMap::<String, String>::new(),
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

fn handle_connection(mut stream: TcpStream, mem_store: &mut MemoryStore) -> io::Result<()> {
    let reader = BufReader::new(stream.try_clone()?);

    for line in reader.lines() {
        let line = line?;

        let split_lines = line
            .split_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();

        let Some((command_name, args)) = split_lines.split_first() else {
            writeln!(stream, "ERR must provide a command")?;
            continue;
        };

        let command = Command::from_str(command_name)
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid command"))?;

        let result: Result<Option<String>, String> = mem_store.handle(&command, args);

        if let Result::Ok(Some(res)) = result {
            writeln!(stream, "{res}")?;
        }

        println!("{}", line);
        println!("{:#?}", mem_store);
    }
    Ok(())
}
