//! 华中科技大学接口技术大作业
use std::time::Duration;
use std::thread;
fn main() {
    println!("Hello, nexys4!");
    let ports = serialport::available_ports().expect("no ports found!");
    ports.iter().for_each(|p| println!("[port] {}", p.port_name));
    let mut port = serialport::new(&ports[0].port_name, 115_200)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("failed to open port!");
    
    let mut read_buf = [0u8; 5];
    let write_buf = ["a".as_bytes(), "b".as_bytes(), "c".as_bytes(), "d".as_bytes(), "e".as_bytes()];
    for s in write_buf.iter() {
        port.write(s).expect("write failed!");
        thread::sleep(Duration::from_secs(1));
        port.read(&mut read_buf).expect("read failed!");
        println!("receive: {}", read_buf[0] as char);
    }
}
