# rew-down
[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

rew-down is a software to help wm users shut down.

!! *Not completed, it is not recommended to use it now* !!


## BUILD

Use Cargo:

```bash
nix-shell -p glib.dev pkg-config zlib cargo pango gdk-pixbuf gtk4 
cargo run
```
Use Nix:

```
nix build --no-update-lock-file
```

## TODO

- [x] shutdown   关机
- [x] logout     注销
- [x] reboot     重启
- [x] hibernate  休眠 
- [x] sleep      睡眠
- [ ] optimize GUI
- [ ] systemd unit

[info](https://www.reddit.com/r/rust/comments/ec59eg/new_rust_library_to_shut_down_reboot_or_log_out/)
