# async-tw-econ

## Description

Rust library provides you a simple asynchronous interface to interconnect with Teeworlds external console.

## Example

Let's say you have Teeworlds server running with `ec_password zohan` and `ec_port 6060` and you want to use it's econ.

```rust
use async_tw_econ::Econ;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut econ = Econ::new();

    econ.connect("127.0.0.1:6060").await?;

    let authed = econ.try_auth("nahoz").await?;
    assert_eq!(authed, false);

    let authed = econ.try_auth("hozan").await?;
    assert_eq!(authed, false);

    let authed = econ.try_auth("zohan").await?;
    assert_eq!(authed, true);

    econ.send_line("echo \"Hi\"").await?;

    econ.fetch().await?;
    
    println!("{}", econ.pop_line()?);

    Ok(())
}
```
