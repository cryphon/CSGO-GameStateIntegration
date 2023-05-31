use std::{
    io::{prelude::*, BufReader},fs,
    net::{TcpListener, TcpStream}
};
use serde_json::{self, Value};

pub fn init_server(){
  let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

  //create thread limit
  for stream in listener.incoming(){
    let stream = stream.unwrap();
    handle_connection(stream);
  }
}

// handle incoming connection
fn handle_connection(mut stream: TcpStream){
  let buff_reader = BufReader::new(&mut stream);
  let request_line = buff_reader.lines().next().unwrap().unwrap();


  //if GET !!! MAKE POST 
  if request_line == "GET / HTTP/1.1" {
    handle_get(&stream);
  }

  if request_line == "POST / HTTP/1.1"{
    handle_post(&stream);
  }
}


// handle GET requests
fn handle_get(mut stream : &TcpStream){
  let status = "HTTP/1.1 200 OK";
    let json: &str = &fs::read_to_string("www/stuff.json").unwrap();
    //create object form json
    let object: Value = serde_json::from_str(json).unwrap();

    let response =
        format!("{status}\r\nContent-Type: application/json\r\n\r\n{object}");
    //let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{object}");
    stream.write_all(response.as_bytes()).unwrap();
}


// handle POST requests
fn handle_post(stream : &TcpStream){
  let object: Value = return_post_object(stream); 
  println!("{:#?}", object["player"]["weapons"]); 
}


fn return_post_object(mut stream: &TcpStream) -> Value{
  let reader = BufReader::new(&mut stream);
    let inner = reader.into_inner();
    let content: &mut Vec<u8> = &mut Vec::new();

    let _res = inner.read_to_end(content);

    //strip text
    let s = String::from_utf8_lossy(content);
    let text = s.as_ref();
    let result = text.replace("\t", "").replace("\n", "").replace(r#"\"#, "");

    let object: Value = serde_json::from_str(result.as_ref()).unwrap();
    return object;
}