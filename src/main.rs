
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::{accept, Message};



fn main() {
    let event_response = r#"
    [
	  "OK",
	  "082155c14942cbe52fcc188711cdce699c812da4532d55af34cc557ae6728b98",
	  true,
	  ""
	]"#;

    let query_response = r#"["QUERY", "082155c14942cbe52fcc188711cdce699c812da4532d55af34cc557ae6728b98",1707200292,1,2, {"Option 1": 1,"Option 2": 2,"Option 3": 3}]"#;

    let query_sids_response = r#"
    [
	  "082155c14942cbe52fcc188711cdce699c812da4532d55af34cc557ae6728b98":"{
	  "title": "I am title",
	  "info": "info info info"
	  }",
	    "082155c14942cbe52fcc188711cdce699c812da4532d55af34cc557ae6728b98":"{
	  "title": "I am title",
	  "info": "info info info"
	  }",
	  "082155c14942cbe52fcc188711cdce699c812da4532d55af34cc557ae6728b98":"{
	  "title": "I am title",
	  "info": "info info info"
	  }"
	]"#;

    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    // let msg = Message::from("adasdasd");

                    let recv: Vec<String> = serde_json::from_str(&*msg.to_string()).unwrap();
                    match recv.first() {
                        Some(st) => {
                            if *st == String::from("EVENT") {
                                websocket.send(Message::from(event_response)).unwrap();
                            } else if *st == String::from("QUERY") {
                                websocket.send(Message::from(query_response)).unwrap();
                            } else if *st == String::from("QUERY_SID") {
                                websocket.send(Message::from(query_sids_response)).unwrap();
                            }
                        }
                        None => websocket.send(Message::from("Error")).unwrap(),
                    };
                }
            }
        });
    }
}
