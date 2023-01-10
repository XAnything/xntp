# xntp
this is a simple and small ntp time client


# example
```rust
use xntp::NtpClient;
fn main(){
    let client = NtpClient::new();
    let res = client.request("ntp.aliyun.com:123");
    println!("{}", res.unix_time);
    println!("{}", res.format_time("%Y-%m-%d %H:%M:%S"));
}
```