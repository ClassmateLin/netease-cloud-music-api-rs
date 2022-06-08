//! 网易云API库
//! # 使用
//! ```rust
//! use ncm_api::{login, user};
//! 
//! #[tokio::main]
//!async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let email = "xxx@163.com";
//!    let password = "password";
//!    let data = login::login_by_email(email, password).await?;
//!    println!("{:?}", data);
//!    let sign_res = user::daily_sign().await?;
//!    println!("sign res:{:?}", sign_res);
//!    Ok(())
//! }
//! ```

#[macro_use]
extern crate lazy_static;
extern crate serde_json;


mod request;
mod crypto;
pub mod login;
pub mod user;
pub mod yunbei;
pub mod musician;
pub mod playlist;