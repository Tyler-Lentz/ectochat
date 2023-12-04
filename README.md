# ectochat

Ectochat is a LAN messenger app inspired by fond childhood memories of Nintendo's [pictochat](https://en.wikipedia.org/wiki/PictoChat). Send messages and images to your friends that are connected to the same WIFI/ethernet connection.

(TODO: put app image here)

## Download

See all releases [here](https://github.com/Tyler-Lentz/ectochat/releases).

Eventually, I plan to set up a github action to compile the app for all platforms (as described [here](https://tauri.app/v1/guides/building/cross-platform/)). For now, however, there will only be releases for Linux systems with the AppImage format.

## Security

Ectochat operates by sending broadcast UDP packets across the local network. In other words, anyone connected to the same network can see everything you send, so don't send anything you wouldn't say out loud in a room!

## Planned Features

- ACK messages so you can see who has seen your message
- Quiet mode, which does not send ACKS or a "Hello World!" message upon joining.
- Send images in messages
- Many chatrooms (under different UDP Ports)
- Robust error handling (so that the app doesn't just crash on error)
    - limit size of text messages
    - remove all unsafe `unwrap` calls in Rust code, and add meaningful error messages through frontend modals.
- Send large files / images in messages
    - probably via multiple messages with seq_nums 

## Build

1. Install the necessary system packages, as described [here](https://tauri.app/v1/guides/getting-started/prerequisites/).
2. From the root of the repository, run the following commands:

```sh
npm install && cd src-tauri
            && cargo install && cd .. 
                             && cargo install tauri-cli
                             && cargo tauri dev
```

3. The above commands only need to be run once. From that point on, to run a development version of the app you can simply run 

```sh
cargo tauri dev
```

from the root level of the repository.