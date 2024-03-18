use std::time::Duration;
use crossbeam_channel::{bounded, tick, Receiver, select};
use anyhow::Result;
use ctrlc;

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {let _ = sender.send(());
    })?;
    Ok(receiver)
}

fn main() -> Result<()> {
    println!("Hello, world!");
    let ctrl_c_events = ctrl_channel()?;
    let ticks = tick(Duration::from_secs(1));

    loop {
        select! {
            recv(ticks) -> _ => {
                println!("Working!")
            }
            recv(ctrl_c_events) -> _ => {
                println!();
                println!("Goodbye!");
                break;
            }
        }
    }
    Ok(())
}
