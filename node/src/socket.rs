use std::net::TcpStream;

use tungstenite::{connect, http::Response, stream::MaybeTlsStream, WebSocket};
use url::Url;

pub struct SocketManger {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
    #[allow(dead_code)]
    response: Response<Option<Vec<u8>>>,
    message_ids: Vec<String>,
    client_id: String,
}
impl SocketManger {
    pub fn new(tcp_server_addr: String) -> Self {
        let (socket, response) = connect(Url::parse(&format!("ws://{}", tcp_server_addr)).unwrap())
            .expect("Can't connect");
        Self {
            socket,
            response,
            message_ids: Vec::new(),
            client_id: uuid::Uuid::new_v4().to_string(),
        }
    }
    pub fn start_listen_message<K: OnMessage>(&mut self, _han: K) {
        loop {
            let msg = self.socket.read_message().expect("Error reading message");
            let msg = match msg {
                tungstenite::Message::Text(s) => s,
                _ => {
                    panic!()
                }
            };
            let data = serde_json::from_str::<WsMessage>(&msg).unwrap();
            if self.message_ids.contains(&data.msg_id) {
                let index = self
                    .message_ids
                    .iter()
                    .position(|x| *x == data.msg_id.clone())
                    .unwrap();
                self.message_ids.remove(index);
                continue;
            }
            let mut util = MessageUtil::new(&mut self.socket, self.client_id.clone());
            K::call(
                MessageInput {
                    data: data.data,
                    client_id: self.client_id.clone(),
                },
                &mut util,
            );
            if util.pushable {
                for msg_id in util.message_ids {
                    self.message_ids.push(msg_id);
                }
            }
        }
    }
    pub fn get_message_util(&mut self) -> MessageUtil {
        MessageUtil::new(&mut self.socket, self.client_id.clone())
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum TWsMessageData {
    String(String),
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct MessageInput {
    pub data: TWsMessageData,
    pub client_id: String,
}

pub trait OnMessage {
    fn call(_message: MessageInput, _util: &mut MessageUtil) {}
}

pub struct MessageUtil<'a> {
    socket: &'a mut WebSocket<MaybeTlsStream<TcpStream>>,
    message_ids: Vec<String>,
    pushable: bool,
    client_id: String,
}

impl<'a> MessageUtil<'a> {
    pub fn new(socket: &'a mut WebSocket<MaybeTlsStream<TcpStream>>, client_id: String) -> Self {
        Self {
            socket,
            message_ids: Vec::new(),
            pushable: false,
            client_id,
        }
    }
    pub fn send_msg(&mut self, data: String) {
        let msg_id = uuid::Uuid::new_v4().to_string();
        self.message_ids.push(msg_id.clone());
        let msg_data = WsMessage {
            msg_id,
            data: TWsMessageData::String(data),
            client_id: self.client_id.clone(),
        };

        self.socket
            .write_message(tungstenite::Message::Text(
                serde_json::to_string(&msg_data).unwrap(),
            ))
            .unwrap();
    }
    #[allow(dead_code)]
    pub fn set_skip_message_from_self(&mut self, push: bool) {
        self.pushable = push;
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
struct WsMessage {
    pub msg_id: String,
    pub client_id: String,
    pub data: TWsMessageData,
}
