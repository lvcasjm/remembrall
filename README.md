# Remembrall

A CLI tool for recording consumed media, built in Rust.

## Disclaimer

I built this tool for myself, for a couple of reasons:
- I wanted to keep track of the media I was consuming, It's fun to look back on.
- I wanted an excuse to learn Rust.

With the above in mind, The only reason this is open source is because I enjoyed
building it and I figure that if someone else wants to use it then why not.

## TODOs
[] List only shows items from the current year, allow for all/any.

___

## Installation

### Prereqs
- Ensure rust installed including `cargo`
- Create database file.

### Building and Installing
- `git clone {repo}` 
- `cd remembrall`
- cargo build
- need to work out how the f someone could build this without the hassle of sqlx 
- cargo install --path .


## Usage

```sh
remembrall
```

### List
```sh
remembrall -l
```

### Create
```sh
remembrall -c
```

### Setup
```sh
remembrall -s
```
