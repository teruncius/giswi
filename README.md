# gis

## Run

```sh
cargo run
```

## Build

```sh
cargo build -r
```

## Install

```sh
sudo cp ./target/release/gis /usr/local/bin/
```

## Config

Create a `.gis.json` file in your home directory.

```json
{
    "work": {
        "name": "John Doe",
        "email": "user@example.com",
        "signingkey": "1234567890ABCEDFGHIJKLMNOPQRSTUVWXYZ1234",
        "gpgsign": true
    }
}
```
