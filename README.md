# WYND Generator

When you generate a new mnemonic, it is very random (must be to be secure), and you
cannot predict the address you will get. However, if you want to get a certain prefix
on your address, you can just try again and again until you get the one you want.

If you have to click buttons on Keplr each time, this can be very slow. This is a simple
Rust program to automate that. It keeps generating new seeds and checking the address
until it finds one that starts with `juno1wynd...`. When it does, it will spit out the
address and the corresponding mnemonic to your terminal (not to a file), for you to
write somewhere secure and import into your keychain (Keplr or Ledger).

## Security Notes

You should be very careful with anything that touches or generates mnemonics, as
a corrupted program (or malware) could later steal everything from your address.
This is quite short and I advice you to review the [source code](./src/main.rs)
to ensure it is not sending your mnemonic to anyone on the internet.

## Usage

(Pre-built binaries coming later, for now, just for devs)

Make sure you have rust installed. If not, please [install rustup](https://rustup.rs) and
then `rustup toolchain install stable`

After that, you just need to clone and build the file. Then run it. The binary should
be about 650kB.

```shell
git clone https://github.com/cosmorama/wynd-gen.git
cd wynd-gen
git checkout v0.1.0 # or latest release
cargo install --path .
```

Ensure you have `export PATH="$HOME/.cargo/bin:$PATH"` in your shell rc file (or run it now),
and you can run this to get a new address in a few minutes:

```shell
time wynd-gen
```

(time is not needed, but nice to see how long it does take. Usually between 1-20 minutes)
