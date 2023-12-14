use serde_json::json;
use std::{net::Ipv4Addr, time::Duration};
use tokio::{net::UdpSocket, time::sleep};

#[tokio::main]
async fn main() {
    let ip = Ipv4Addr::new(192, 168, 50, 145);
    let length = 159;

    // https://kno.wled.ge/interfaces/udp-realtime/
    let mut packet = vec![0u8; 4 + 3 * length];
    packet[0] = 4; // DNRGB
    packet[1] = 1; // 1 second before led strip will switch back to normal mode
    packet[2] = 0;
    packet[3] = 0;

    for led in 0..length {
        let color = [255, 0, 128];
        for c in 0..3 {
            packet[4 + led * 3 + c] = color[c];
        }
    }

    let socket = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    socket.send_to(&packet, (ip, 21324)).await.unwrap();

    sleep(Duration::from_millis(100)).await;

    let http = reqwest::Client::new();
    http.post(&format!("http://{ip}/json/state"))
        .json(&json!({
            "on": false,
        }))
        .send()
        .await
        .unwrap();
}
