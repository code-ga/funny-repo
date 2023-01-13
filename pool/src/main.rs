use ws::WebSocket;

struct Handler {
    pub(crate) ws: ws::Sender,
}
impl ws::Handler for Handler {
    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        println!("New client connect : {:#?}", shake.peer_addr.unwrap());
        Ok(())
    }
    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        dbg!(&msg);
        self.ws.broadcast(msg)?;
        Ok(())
    }
    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        println!(
            "Client disconnect code : {:#?} , reason : {:#?}",
            code, reason
        );
    }
}

fn main() {
    let ws_handler = |out: ws::Sender| Handler { ws: out };
    let ws = WebSocket::new(ws_handler).unwrap();
    std::thread::spawn(move || {
        ws.listen("0.0.0.0:8080").unwrap();
    }).join().unwrap();
}
