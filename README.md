# kth

Command line tool that filters stdin lines for those who love tools that just work

![Demo](https://user-images.githubusercontent.com/62714153/210451542-eb5e37d0-8e32-4487-a62e-c50c2a2ea9b1.gif)

This tool is inspired by [nth](https://github.com/coledot/nth)

## How to install?

Just execute the command bellow and you are ready to go:
```sh
wget -qO- https://raw.githubusercontent.com/TiagoCavalcante/kth/main/scripts/install.sh | bash
```

## Build yourself
Building it yourself is very easy:
```sh
git clone https://github.com/TiagoCavalcante/kth
cargo build --release
./target/release/kth 1 2 3
```
