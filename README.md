# gitdonewithit

A simplistic tool for using git

## Dependencies

The program works mostly without any external dependencies, except
from [Git](https://github.com/git/git) of course, but the gii log
command requires an installation of
[neovim](https://github.com/neovim/neovim) present.

## Installation Method

Clone the repo.

```{
git clone https://github.com/th0jensen/gitdonewithit.git && cd gitdonewithit
```

Make sure you have Rust installed

```{
rustc --version && cargo --version
```

Install Rust it's not installed

```{
curl https://sh.rustup.rs -sSf | sh
```

Compile and install using cargo

```{
cargo build --release && cargo install --path .
```

Run the program and suffer:

```{
gii help
```
