use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::{accept, Message};

use serde_json::{Value};

fn main() {
    let event_response = r#"
    [
	  "OK",
	  "082155c14942cbe52fcc188711cdce699c812da4532d55af34cc557ae6728b98",
	  true,
	  ""
	]"#;

    let query_response = r#"["QUERY", "082155c14942cbe52fcc188711cdce699c812da4532d55af34cc557ae6728b98","I am title","info info info", 1707200292,1,2, {"Option 1": 1,"Option 2": 2,"Option 3": 3}]"#;

    let query_sids_response = r#"
    [
	  {
	  "id": "082155c14942cbe52fcc188711cdce699c812da4532d55af34cc557ae6728b98",
	  "title": "I am title",
	  "info": "info info info"
	  },
	 {
	  "id": "082155c14942cbe52fcc188711cdce699c812da4532d55af34cc557ae6728b98",
	  "title": "I am title",
	  "info": "info info info"
	  },
	  {
	  "id": "082155c14942cbe52fcc188711cdce699c812da4532d55af34cc557ae6728b98",
	  "title": "I am title",
	  "info": "info info info"
	  }
	]"#;

    let server = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    let parsed: Value = read_json(&*msg.to_string());

                    let st = parsed[0].to_string();

                    match &st as &str {
                        "\"EVENT\"" => {
                            websocket.send(Message::from(event_response)).unwrap()
                        },
                        "\"QUERY\"" => { websocket.send(Message::from(query_response)).unwrap() },
                        "\"QUERY_SID\"" => { websocket.send(Message::from(query_sids_response)).unwrap() },
                        _ =>{websocket.send(Message::from("ERROR")).unwrap()}
                    }
                }
            }
        });
    }
}

fn read_json(raw_json: &str) -> Value {
    let parsed: Value = serde_json::from_str(raw_json).unwrap();
    return parsed;
}