use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::{accept, Message};

use serde_json::{Value};
use serde::{Deserialize, Serialize};
use std::string::String;
use std::sync::mpsc;
use std::thread;

fn main() {
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

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct PullMessageResult {
        pub status: String,
        pub message: String,
        pub total: u64,
        pub list: [ReceiveMessage; 2],
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct PullMessageResponse {
        pub id: String,
        pub jsonrpc: String,
        pub result: PullMessageResult,
    }

    let server1 = TcpListener::bind("0.0.0.0:8080").unwrap();
    let server2 = TcpListener::bind("0.0.0.0:8081").unwrap();

    let (atx, brx) = mpsc::channel();
    let (btx, arx) = mpsc::channel();


    let handle1 = spawn(move || {

        for stream in server1.incoming() {
            let mut websocket = accept(stream.unwrap()).unwrap();
            let mut current_account = "".to_string();
            loop {
                let msg = websocket.read().unwrap();


                if let Result::Ok(received) = arx.try_recv() {
                    println!("收到消息：{:?}", received);
                    println!("receive from B");
                    websocket.send(Message::from(
                        serde_json::to_string(&received).unwrap().as_str()
                    )).unwrap();
                }
                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    let parsed: Value = read_json(&*msg.to_string());
                    let mut st = parsed["method"].to_string();
                    match &st as &str {
                        "\"bootstrap_getNode\"" => {
                            let res1 = BootstrapGetNodeResult {
                                url: "ws://52.221.181.98:8080".to_string(),
                                name: "Alice".to_string(),
                            };
                            let res2 = BootstrapGetNodeResult {
                                url: "ws://52.221.181.98:8081".to_string(),
                                name: "Bob".to_string(),
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

                            let response = NodeConnectNodeResponse {
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
                            let response = NodeConnectNodeResponse {
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

                            let response = AccountReceiveMessage {
                                id: parsed["id"].to_string(),
                                jsonrpc: "2.0".to_string(),
                                method: "account_receiveMessage".to_string(),
                                result: AccountReceiveMessageRes {
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
                            println!("send to B");
                            atx.send(response).unwrap();
                        }
                        "\"account_pullMessage\"" => {
                            let resMessage = ReceiveMessage {
                                from: "fake".to_string(),
                                to: "fake".to_string(),
                                message: "fake".to_string(),
                                sign: "fake".to_string(),
                            };
                            let resMessage2 = ReceiveMessage {
                                from: "fake".to_string(),
                                to: "fake".to_string(),
                                message: "fake".to_string(),
                                sign: "fake".to_string(),
                            };

                            let response = PullMessageResponse {
                                id: parsed["id"].to_string(),
                                jsonrpc: "2.0".to_string(),
                                result: PullMessageResult {
                                    status: "success".to_string(),
                                    message: "".to_string(),
                                    total: 2,
                                    list: [resMessage, resMessage2],
                                },
                            };


                            websocket.send(Message::from(serde_json::to_string(&response).unwrap().as_str())).unwrap()
                        }
                        "\"ping\"" => { websocket.send(Message::from("pong")).unwrap() }

                        _ => { websocket.send(Message::from("ERROR")).unwrap() }
                    }
                }
            }
        }
    });

    let handle2 = spawn(move || {
        for stream in server2.incoming() {
            let mut websocket = accept(stream.unwrap()).unwrap();
            let mut current_account = "".to_string();

            loop {
                let msg = websocket.read().unwrap();



                if let Result::Ok(received) = brx.try_recv() {
                    println!("收到消息：{:?}", received);
                    println!("receive from A");
                    websocket.send(Message::from(
                        serde_json::to_string(&received).unwrap().as_str()
                    )).unwrap();
                }
                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    let parsed: Value = read_json(&*msg.to_string());
                    let mut st = parsed["method"].to_string();
                    match &st as &str {
                        "\"bootstrap_getNode\"" => {
                            let res1 = BootstrapGetNodeResult {
                                url: "ws://52.221.181.98:8080".to_string(),
                                name: "Alice".to_string(),
                            };
                            let res2 = BootstrapGetNodeResult {
                                url: "ws://52.221.181.98:8081".to_string(),
                                name: "Bob".to_string(),
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

                            let response = NodeConnectNodeResponse {
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
                            let response = NodeConnectNodeResponse {
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

                            let response = AccountReceiveMessage {
                                id: parsed["id"].to_string(),
                                jsonrpc: "2.0".to_string(),
                                method: "account_receiveMessage".to_string(),
                                result: AccountReceiveMessageRes {
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
                            // if current_account == (parsed["params"]["to"].to_string()) {
                            //     let response = AccountReceiveMessage{
                            //         id: parsed["id"].to_string(),
                            //         jsonrpc: "2.0".to_string(),
                            //         method: "account_receiveMessage".to_string(),
                            //         result: AccountReceiveMessageRes{
                            //             status: "success".to_string(),
                            //             message: "".to_string(),
                            //             receiveMessage: ReceiveMessage {
                            //                 from: parsed["params"]["from"].to_string(),
                            //                 to: parsed["params"]["to"].to_string(),
                            //                 message: parsed["params"]["message"].to_string(),
                            //                 sign: parsed["params"]["sign"].to_string(),
                            //             },
                            //         },
                            //     };
                            //     websocket.send(Message::from(
                            //         serde_json::to_string(&response).unwrap().as_str()
                            //     )).unwrap();
                            // }
                            btx.send(response).unwrap()
                        }
                        "\"account_pullMessage\"" => {
                            let resMessage = ReceiveMessage {
                                from: "fake".to_string(),
                                to: "fake".to_string(),
                                message: "fake".to_string(),
                                sign: "fake".to_string(),
                            };
                            let resMessage2 = ReceiveMessage {
                                from: "fake".to_string(),
                                to: "fake".to_string(),
                                message: "fake".to_string(),
                                sign: "fake".to_string(),
                            };

                            let response = PullMessageResponse {
                                id: parsed["id"].to_string(),
                                jsonrpc: "2.0".to_string(),
                                result: PullMessageResult {
                                    status: "success".to_string(),
                                    message: "".to_string(),
                                    total: 2,
                                    list: [resMessage, resMessage2],
                                },
                            };


                            websocket.send(Message::from(serde_json::to_string(&response).unwrap().as_str())).unwrap()
                        }
                        "\"ping\"" => { websocket.send(Message::from("pong")).unwrap() }

                        _ => { websocket.send(Message::from("ERROR")).unwrap() }
                    }
                }
            }
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}


fn read_json(raw_json: &str) -> Value {
    let parsed: Value = serde_json::from_str(raw_json).unwrap();
    return parsed;
}