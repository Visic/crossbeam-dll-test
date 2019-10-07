#[no_mangle]
pub fn start(receiver: crossbeam_channel::Receiver<i32>) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        println!("{:?}", receiver.recv());
    })
}