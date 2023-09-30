use crate::models::links;

use tokio::task;
use mavlink;

impl links::LinkType {
    pub fn to_mavlink(&self) -> String {
        match self {
            links::LinkType::Udp { address, port } => {
                return format!("udpout:{}:{}", address, port)
            },
            links::LinkType::Tcp { address, port } => {
                return format!("tcpout:{}:{}", address, port)
            },
            links::LinkType::Serial { port, baud_rate } => {
                return format!("serial:{}:{}", port, baud_rate)
            },
        }
    }
}

impl links::ProtocolVersion {
    pub fn to_mavlink(&self) -> mavlink::MavlinkVersion {
        match self {
            links::ProtocolVersion::MavlinkV1 => return mavlink::MavlinkVersion::V1,
            links::ProtocolVersion::MavlinkV2 => return mavlink::MavlinkVersion::V2,
        }
    }
}

pub async fn connect(link_type: &links::LinkType, protocol: &links::ProtocolVersion) -> std::io::Result<()> {
    println!("MAVLink connect: {:?}:{:?}", &link_type, &protocol);

    let mut connection = mavlink::connect::<mavlink::common::MavMessage>(&link_type.to_mavlink())?;

    connection.set_protocol_version(protocol.to_mavlink());

    let listening_task = tokio::task::spawn(async move {
        loop {
            println!("Recv!");
            match connection.recv() {
                Ok((header, msg)) => {
                    println!("Got mavlink message: {:?}:{:?}", &header, &msg);
                },
                Err(err) => {
                    println!("Got mavlink error: {:?}", &err);
                }
            }
        }
    });

    // TODO: sending_task, use tokio::select!

    listening_task.await?;

//     #[tokio::main]
// async fn main() {
//     // Cell is an acceptable complication when accessing the data.
//     let val = std::cell::Cell::new(1);
//     tokio::select! {
//       _ = async {loop {
//         println!(".{}", val.get());
//         sleep(Duration::from_millis(200)).await;
//       }} => {},
//       _ = async {loop {
//         println!("Starting slow operation...");
//         // The problem: During this await the dots are not printed.
//         sleep(Duration::from_secs(1)).await;
//         val.set(val.get() + 1);
//         println!("...done");
//         sleep(Duration::from_secs(3)).await;
//       }} => {},
//     }
// }

    Ok(())
}