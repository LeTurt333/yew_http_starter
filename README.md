# Yew HTTP starter

### What is this?

A [Yew](https://github.com/yewstack) starter template with a simple HTTP message built in.

App.rs (where a majority of the code is) has some useful comments to understand the structure of the template.

#### What you will need

 - [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html)
 - [Trunk](https://crates.io/crates/trunk) - Wasm web app bundler 

---

### Usage

Set target:
> `$ rustup target add wasm32-unknown-unknown`

Clone this repo: 
> `$ git clone https://github.com/LeTurt333/yew_http_starter`

In the root of the repo you just cloned, run:
> `$ trunk serve`

Open a browser window & go to:
> `http://127.0.0.1:8080/`

**Note: Make sure to plug in your own api if you want it to work right away!**
