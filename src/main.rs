use std::thread::JoinHandle;
use libloading::{Library, Symbol};
use crossbeam_channel::{Receiver, unbounded};

fn main() {
    let lib = Library::new("plugin.dll").unwrap();
    let start_fn = unsafe { lib.get::<Symbol<fn(Receiver<i32>) -> JoinHandle<()>>>(b"start") }.unwrap();

    let (sender, receiver) = unbounded();
    let join_handle = start_fn(receiver);
    
    std::thread::sleep(std::time::Duration::from_millis(100)); //force this thread to wait until the plugin is receiving, this exposes the bug

    sender.send(0).unwrap();
    join_handle.join().unwrap();
}