use std::time::Duration;

use crossbeam_channel::{bounded, Receiver, tick, select};
use tokio::{time::sleep};

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        sender.send(()).expect("Error while sending");
    })?;
    Ok(receiver)
}

pub async fn start_tmp_test() {
    let ctrlc_c_events = ctrl_channel().expect("Could not create channel");
    let ticks = tick(Duration::from_secs(3));

    loop {
        select! {
            recv(ticks) -> _ => {
                println!("Doing something...");
            }
            recv(ctrlc_c_events) -> _event => {
                println!("received SIGINT event");
                break;
            }
        }
    }
}