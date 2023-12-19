# ectochat

Ectochat is a LAN messenger app inspired by fond childhood memories of Nintendo's [pictochat](https://en.wikipedia.org/wiki/PictoChat). Send messages and images to your friends that are connected to the same local area network.

## Download

See all releases [here](https://github.com/Tyler-Lentz/ectochat/releases).

## Planned Features

- image messages
- audio messages
- send message to specific person, instead of everyone
- better UI to view all of the people you are currently chatting with

## Security

Messages sent via Ectochat are not encrypted, so do not use this app to send any sensitive information!

## Network

There are two major parts to how this app sends messages on the network.

First, there are broadcasts. These are broadcast UDP datagrams (i.e. they are sent to everyone on the LAN) that are periodically sent by each host which announce one's presence.

Once your app detects that there is another host on the network using Ectochat, a TCP stream will be established with that host. This is the second part of how the networking works. The host with the greater UID initiates the connection, while the other accepts. Each side starts by sending a "Hello" message, which associates their UID with a name and profile picture.

From then on out, every message that you send will be placed in each active TCP stream you have open. When a host leaves the app, they send a "Goodbye" message to all of their active TCP streams, before terminating the connection. This allows the other hosts to gracefully display a message saying that the host has left the chat room.

If a TCP connection drops, for whatever reason, then the app will terminate the connection itself and assume that the other host either crashed, killed the process, or ended their application in some other nonstandard way. This will display a message saying that a connection has been dropped.

While TCP itself guarantees reliability via ACKs, there are also "Ack" messages that are sent by each client to verify that the message was correctly received and displayed on the other host's screen. These are shown by hovering over the eyeball icon to the right of messages.

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