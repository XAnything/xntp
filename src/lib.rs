//! # XNtp
//!
//! `xntp` is a client of ntp to get net time and format
//!
//!
//! # Example
//! ```rust
//! use xntp::NtpClient;
//! fn main(){
//!     let client = NtpClient::new();
//!     let res = client.request("ntp.aliyun.com");
//!     println!("{}", res.unix_time);
//!     println!("{}", res.format_time("%Y-%m-%d %H:%M:%S"));
//! }
//!
//! ```


use std::net::UdpSocket;
use byteorder::{ BigEndian, ReadBytesExt };
use std::io::{ Cursor, Seek, SeekFrom };
use chrono::prelude::*;
use anyhow::Result;

pub struct NtpClient;
impl NtpClient {
    pub fn new() -> NtpClient {
        NtpClient
    }
    pub fn request(self, server: &str) -> Result<Response> {
        let client = UdpSocket::bind("0.0.0.0:0")?;
        client.connect(format!("{server}:123"))?;
        let mut request_data = vec![0;48];
        request_data[0] = 0x1b;
        client.send(&request_data)?;
        let mut buf = [0; 48];
        client.recv(&mut buf)?;
        let ntp_second = self.unpack_ntp_data(&buf)?;
        let unix_second = ntp_second - 2208988800;
        let response = Response {
            unix_time: unix_second,
        };
        Ok(response)
    }
    fn unpack_ntp_data(self, buffer: &[u8; 48]) -> Result<u64> {
        let mut reader = Cursor::new(buffer);
        reader.seek(SeekFrom::Current(40))?;
        let ntp_second = reader.read_u32::<BigEndian>()?;
        Ok(u64::from(ntp_second))
    }
}

pub struct Response {
    pub unix_time: u64,
}

impl Response {
    pub fn format_time(self, fmt: &str) -> String {
        // time_format::strftime_utc(fmt, self.unix_time as i64).unwrap()
        let dt = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(
                self.unix_time as i64,
                ((self.unix_time % 1000) as u32) * 1_000_000
            ).unwrap(),
            Utc
        );
        let shanghai = FixedOffset::east_opt(8 * 3600).unwrap();
        format!("{}", dt.with_timezone(&shanghai).format(fmt))
    }
}

#[test]
fn test() {
    let client = NtpClient::new();
    let res = client.request("ntp.aliyun.com").unwrap();
    println!("{}", res.unix_time);
    println!("{}", res.format_time("%Y-%m-%d %H:%M:%S"));
}
