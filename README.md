
Build:
```sh
cargo build --release
```

Test run:
```sh
cd target/release && ./rust-cli-timer 10s
```

---

Copy binary for further usage:
```sh
cd target/release
cp rust-cli-timer ~/.local/bin/timer
```

Add to your .bashrc:
```bash
timer () {
    ~/.local/bin/timer $1
}
```

--- 

Usage:
```sh
timer 10s
timer 120s
timer 3m
timer 12h
timer 1h1m1s
```
