# poolboy
![CI tests](https://github.com/caheredia/poolboy/actions/workflows/rust.yml/badge.svg)

A convenient way to display, in your browser, stats from your local [Monero P2Pool](https://github.com/SChernykh/p2pool/tree/master), a decentralized pool for Monero mining.

`poolboy` is a tiny (~ 200 lines of code) web server that renders a static page with stats from P2Pool [data-api](https://github.com/SChernykh/p2pool/blob/master/docs/COMMAND_LINE.MD). 

![poolboy](figures/screenshot.png)

## Installation

```console
cargo install --git https://github.com/caheredia/poolboy
```

## Run the server
Execute with crontab on server running P2Pool and poolboy
```
@reboot $HOME/.cargo/bin/poolboy --data-dir /path/to/p2pool-data-api-dir
```
or 
in a terminal 
```console
poolboy --data-dir /path/to/p2pool-data-api-dir
```

## Connecting to the server
Navigate to [http://localhost:3000](http://localhost:3000) or the IP address of your server, e.g., [http://192.168.10.182:3000](http://192.168.10.182:3000).

## Options
```console
❯ poolboy --help
A tiny web server for Monero P2Pool stats

Usage: poolboy --data-dir <DIR>

Options:
  -d, --data-dir <DIR>  Data directory containing P2Pool data
  -h, --help            Print help
  -V, --version         Print version
```

## Donations

If you would like to support this project, you're welcome to send XMR to the following address:
```
88fBbt2Dgmu3X4FDow1pURUXZSiwT3gkVcpDtPC9vL9T9EKDZfeYqaUVZcKiA961TjK4oGF6sHL46Nn6DzWjayo7AdPoLQv 
```
