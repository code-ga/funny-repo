use std::time::Duration;

mod socket;
fn main() {
    // let mut tcp_server_addr = "".to_string();
    // std::io::stdin().read_line(&mut tcp_server_addr).unwrap();
    // tcp_server_addr = tcp_server_addr.trim().to_string();
    let tcp_server_addr = "0.0.0.0:8080".to_string();

    let mut manger = socket::SocketManger::new(tcp_server_addr);
    manger.get_message_util().send_msg("siuuu".to_owned());
    std::thread::spawn(move || manger.start_listen_message(MessageHandler))
        .join()
        .unwrap();
}

struct MessageHandler;
impl socket::OnMessage for MessageHandler {
    fn call(message: socket::MessageInput, manger: &mut socket::MessageUtil) {
        dbg!(&message);
        manger.send_msg("siuuuu".to_string());
        std::thread::sleep(Duration::from_secs(1));
    }
}
