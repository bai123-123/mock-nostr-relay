use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::{accept, Message};

use serde_json::{Value};
use serde::{Deserialize, Serialize};
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

    let account_pullMessage_response = r#"
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

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Params {
        pub from: Option<String>,
        pub to: Option<String>,
        pub address: Option<String>,
        pub sign: Option<String>,
        pub message: Option<String>,
        pub length: Option<String>,
    }
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Request {
        pub id: u64,
        pub jsonrpc: String,
        pub method: String,
        pub params: Params,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct BootstrapGetNodeResult {
        pub url: String,
        pub name: String,
    }
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct BootstrapGetNodeResponse {
        pub id: String,
        pub jsonrpc: String,
        pub result: [BootstrapGetNodeResult; 2],
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct NodeConnectNodeResult {
        status: String,
        message: String,
    }
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct NodeConnectNodeResponse {
        pub id: String,
        pub jsonrpc: String,
        pub result: NodeConnectNodeResult,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct ReceiveMessage {
        pub from: String,
        pub to: String,
        pub message: String,
        pub sign: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct AccountReceiveMessageRes {
        pub status: String,
        pub message: String,
        pub receiveMessage: ReceiveMessage,
    }
    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct AccountReceiveMessage {
        pub id: String,
        pub jsonrpc: String,
        pub method: String,
        pub result: AccountReceiveMessageRes,
    }

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

                    let mut st = parsed["method"].to_string();


                    println!("{}", st);


                    match &st as &str {
                        "\"bootstrap_getNode\"" => {
                            let res1 = BootstrapGetNodeResult {
                                url: "wss://google.com".to_string(),
                                name: "Google".to_string(),
                            };
                            let res2 = BootstrapGetNodeResult {
                                url: "wss://aws.com".to_string(),
                                name: "AWS".to_string(),
                            };
                            let response = BootstrapGetNodeResponse {
                                id: parsed["id"].to_string(),
                                jsonrpc: "2.0".to_string(),
                                result: [res1, res2],
                            };

                            websocket.send(Message::from(
                                serde_json::to_string(&response).unwrap().as_str()
                            )).unwrap()
                        }
                        "\"node_connectNode\"" => {
                            current_account = parsed["params"]["address"].to_string();

                            let response = NodeConnectNodeResponse{
                                id: parsed["id"].to_string(),
                                jsonrpc: "2.0".to_string(),
                                result: NodeConnectNodeResult {
                                    status: "success".to_string(),
                                    message: "".to_string(),
                                },
                            };

                            websocket.send(Message::from(
                                serde_json::to_string(&response).unwrap().as_str()
                            )).unwrap();
                        }
                        "\"account_sendMessage\"" => {

                            let response = NodeConnectNodeResponse{
                                id: parsed["id"].to_string(),
                                jsonrpc: "2.0".to_string(),
                                result: NodeConnectNodeResult {
                                    status: "success".to_string(),
                                    message: "".to_string(),
                                },
                            };


                            websocket.send(Message::from(
                                serde_json::to_string(&response).unwrap().as_str()
                            )).unwrap();
                            if current_account == (parsed["params"]["to"].to_string()) {

                                let response = AccountReceiveMessage{
                                    id: parsed["id"].to_string(),
                                    jsonrpc: "2.0".to_string(),
                                    method: "account_receiveMessage".to_string(),
                                    result: AccountReceiveMessageRes{
                                        status: "success".to_string(),
                                        message: "".to_string(),
                                        receiveMessage: ReceiveMessage {
                                            from: parsed["params"]["from"].to_string(),
                                            to: parsed["params"]["to"].to_string(),
                                            message: parsed["params"]["message"].to_string(),
                                            sign: parsed["params"]["sign"].to_string(),
                                        },
                                    },
                                };



                                websocket.send(Message::from(
                                    serde_json::to_string(&response).unwrap().as_str()
                                )).unwrap();



                            }



                        }
                        "\"account_pullMessage\"" => { websocket.send(Message::from(account_pullMessage_response)).unwrap() }
                        "\"ping\"" => { websocket.send(Message::from("pong")).unwrap() }

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