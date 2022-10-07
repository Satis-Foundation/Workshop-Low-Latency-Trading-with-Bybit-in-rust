# Low Latency Trading with Bybit in Rust

## Preparation (MUST DO BEFORE COME!!!!)
1. Docker installed
    - link: https://docs.docker.com/engine/install/
1. Clone or download this repo
    - git clone: `git clone https://github.com/Satis-Foundation/Workshop-Low-Latency-Trading-with-Bybit-in-rust.git`
    - Download: https://github.com/Satis-Foundation/Workshop-Low-Latency-Trading-with-Bybit-in-rust/archive/refs/heads/main.zip
1. **Within cloned dir** 
    - run `docker build -t low-latency-trading-with-bybit-in-rust .` 
1. Sign up a Bybit testnet account
    - link: https://testnet.bybit.com/
1. Enable 2FA in setting

## (Suggested) Additional setup
- Setup Bybit testnet api key
- Install VS code with rust-analyzer extension
    - VS code: https://code.visualstudio.com/download
    - rust-analyzer extension: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
    - Better TOML : https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml
    - crates : https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates
    - Error Lens : https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens
## Durning workshop start env
- Run `docker run -it --rm -v $PWD/src/:/home/src low-latency-trading-with-bybit-in-rust` in the dir
