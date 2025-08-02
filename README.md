# Rust Pass

A simple Rust password manager inspired by **GNU Pass**.

## Usage

### Initialize the password vault with your default GPG key

```shell
cargo run -- init passwords
```

### Insert a password 
```shell
cargo run -- insert <password_name>
```
### Generate a new password and copy it with wl-copy
```shell
cargo run -- generate password
```
### Show  a stored password 
```shell
cargo run -- password
```




