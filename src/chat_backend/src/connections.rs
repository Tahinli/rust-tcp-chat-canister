use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, self};
use std::env;
use std::str::from_utf8;
use candid::{CandidType, Deserialize};



thread_local! 
    {
        pub static MESSAGES: Vec<Message> = vec![];
    }

#[derive(CandidType, Deserialize, Clone)]
pub struct Message
    {
        message: String,
    }

pub enum EnvArg
    {
        Client,
        Server,
        Fail,
    }
impl EnvArg 
    {
        pub fn c_connect() -> bool
            {
                let mut messages:Vec<Message> = vec![];
                match TcpStream::connect("localhost:2121")
                    {
                        Ok(mut socket) =>
                            {
                                println!("Connected");
                                let stay = true;
                                while stay
                                    {
                                        socket.write(take_string().as_bytes()).unwrap();
                                        let mut data = [0 as u8; 1024];
                                        match socket.read(&mut data)
                                            {
                                                Ok(_) => 
                                                    {
                                                        if data[0] != 0
                                                            {
                                                                println!("{}", from_utf8(&data).unwrap());
                                                                let message = Message
                                                                    {
                                                                        message: from_utf8(&data).unwrap().to_string(),
                                                                    };
                                                                messages.push(message);
                                                            }
                                                    }
                                                Err(e) =>
                                                    {
                                                        println!("Failed to receive data: {}", e);
                                                        return false;
                                                    }
                                            }
                                    }
                            }
                        Err(e) =>
                            {
                                println!("Failed to connect: {}", e);
                                return false;
                            }
                    }
                return true;
            }

        pub fn s_connect() -> bool
            {
                let mut messages:Vec<Message> = vec![];
                let socket = TcpListener::bind("localhost:2121");
                for stream in socket.expect("Can't Check Connections").incoming()
                    {
                        match stream
                            {
                                Ok(mut stream) =>
                                    {
                                        let stay = true;
                                        while stay
                                            {
                                                let mut data = [0 as u8; 1024];
                                                match stream.read(&mut data)
                                                    {
                                                        Ok(a) => 
                                                            {
                                                                if a == 0
                                                                    {
                                                                        println!("Connection Closed");
                                                                        return false;
                                                                    }
                                                                println!("{}", stream.write(&data).unwrap());
                                                                let message = Message
                                                                    {
                                                                        message: from_utf8(&data).unwrap().to_string(),
                                                                    };
                                                                messages.push(message);
                                                            }
                                                        Err(e) =>
                                                            {
                                                                println!("Failed to Read: {}", e);
                                                                return false;
                                                            }
                                                    }
                                            }
                                    }
                                Err(e) =>
                                    {
                                        println!("Something Happened: {}", e);
                                    }
                            }
                    }
                
                return true;
            }
    }

pub fn take_arg() -> EnvArg
    {
        let args:Vec<String> = env::args().collect();
        match args.len()
            {
                1 => println!("Give an argument"),
                2 => 
                    {
                        if args[1] == "c"
                            {
                                return EnvArg::Client;
                            }
                        else if args[1] == "s"
                            {
                                return EnvArg::Server;
                            }
                        else 
                            {
                                println!("Give valid argument, c or s");
                            }
                    },
                _ => println!("Only one argument is allowed"),
            }
        return EnvArg::Fail;
    }

fn take_string() -> String
    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to Read from Console");
        input
    }

fn client() 
    {
        println!("Client");
        if EnvArg::c_connect() != true
            {
                println!("Client Exit");
            }
    }
fn server()
    {
        println!("Server");
        if EnvArg::s_connect() != true
            {
                println!("Server Exit");
            }
    }
pub fn start()
    {
        println!("Hello, world!");
        match take_arg() 
            {
                EnvArg::Client => client(),
                EnvArg::Server => server(),
                EnvArg::Fail => return
            }
    }