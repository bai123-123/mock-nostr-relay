use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::{accept, Message};

use serde_json::{Value};

use std::string::String;
fn main() {
    let bootstrap_getNode_response = r#"
    {
        "jsonrpc": "2.0",
        "result": [
            {
                "url": "wss://google.com",
                "name": "Google"
            },
             {
                "url": "wss://aws.com",
                "name": "AWS"
            }
        ],
        "id": 949
    }
    "#;

    let node_connectNode_response = r#"
    {
        "jsonrpc": "2.0",
        "result": {
            "status": "success",
            "message": "",
        }
        "id": 949
    }
    "#;

    let account_sendMessage_response = r#"
    {
        "jsonrpc": "2.0",
        "result": {
            "status": "success",
            "message": "",
        }
        "id": 949
    }
    "#;

    let  account_pullMessage_response = r#"
   {
        "jsonrpc": "2.0",
        "result": {
            "status": "success",
            "message": "",
            "total": "100",
            "list": [
            {
                "from": "from",
                "to": "to",
                "message": "message",
                "sign": "sign",
               },
              {
                "from": "from",
                "to": "to",
                "message": "message",
                "sign": "sign",
               },
            ]
        }
        "id": 949
    }
    "#;


    let receive_response = r#"
    {
        "jsonrpc": "2.0",
        "method": "account_receiveMessage",
        "result": {
            "status": "success",
            "message": "",
            "receiveMessage": {
                        "from": "from",
                        "to": "to",
                        "message": "message",
                        "sign": "sign",
                    }
        }
        "id": 949
    }
    "#;






    let server = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in server.incoming() {

        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            let mut current_account = "".to_string();
            loop {
                let msg = websocket.read().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    let parsed: Value = read_json(&*msg.to_string());

                    let st = parsed["method"].to_string();

                    println!("{}", st);



                    match &st as &str {
                        "\"bootstrap_getNode\"" => {
                            websocket.send(Message::from(bootstrap_getNode_response)).unwrap()
                        }
                        "\"node_connectNode\"" => {
                            current_account = parsed["params"]["address"].to_string();
                            websocket.send(Message::from(node_connectNode_response)).unwrap();
                        }
                        "\"account_sendMessage\"" => {
                            websocket.send(Message::from(account_sendMessage_response)).unwrap();
                            if current_account == (parsed["params"]["to"].to_string()){
                                websocket.send(Message::from(receive_response)).unwrap();
                            }


                        }
                        "\"account_pullMessage\"" => { websocket.send(Message::from(account_pullMessage_response)).unwrap() }


                        _ => { websocket.send(Message::from("ERROR")).unwrap() }
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