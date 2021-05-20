//! 串口数据传输
use serialport::SerialPort;
use serialport::Result;
use std::time::Duration;

/// 串口抽象
pub struct Nexys4Serial {
    pub port: Box<dyn SerialPort>,
    previous_temp: u8
}

unsafe impl Sync for Nexys4Serial {}

impl Nexys4Serial {
    /// 第一个可用的串口
    pub fn first_available(baud_rate: u32) -> Result<Self> {
        let ports = serialport::available_ports()?;
        let port = serialport::new(&ports[0].port_name, baud_rate)
            .timeout(Duration::from_millis(10))
            .open()?;
        Ok(
            Nexys4Serial {
                port,
                previous_temp: 0
            }
        )
    }

    /// 获得波特率
    pub fn baud_rate(&self) -> u32 {
        self.port.baud_rate().expect("failed to get baud rate")
    }

    /// 设置波特率
    pub fn set_baud_rate(&mut self, baud_rate: u32) {
        self.port.set_baud_rate(baud_rate).expect("failed to set baud rate");
    }

    /// 读取一个字节
    /// 
    pub fn read_one_byte(&mut self) -> Result<u8> {
        let mut buf = [0u8];
        let bytes = self.port.read(&mut buf)?;
        Ok(buf[0])
    }

    /// 写一个字节
    /// 
    pub fn write_one_byte(&mut self, byte: u8) -> Result<()> {
        let buf = [byte];
        self.port.write(&buf)?;
        Ok(())
    }


    /// 读取串口数据到缓冲区
    /// 返回读取的字节数
    pub fn read_to_buf(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.port.read(buf)
    }

    /// 从缓冲区中写数据到串口
    pub fn write_to_buf(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.port.write(buf)?;
        Ok(())
    }

    /// 更新温度值
    pub fn update_temp(&mut self, temp: u8) {
        self.previous_temp = temp;
    }

    /// 获得上一次温度值
    pub fn prev_temp(&self) -> u8 {
        self.previous_temp
    }
}