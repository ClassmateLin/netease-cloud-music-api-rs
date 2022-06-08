# netease-cloud-music-api-rs

Rust语言版本的网易云API.

## 配置

Cargo.toml添加依赖

```toml
[dependencies]
ncm_api = {version="^0.1.1"}
tokio = {version="^1"}
```

## 使用

```rust
use ncm_api::{login, user};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let email = "xxx@163.com";
    let password = "password";
    let data = login::login_by_email(email, password).await?;
    println!("{:?}", data);
    let sign_res = user::daily_sign().await?;
    println!("sign res:{:?}", sign_res);
    Ok(())
}
```