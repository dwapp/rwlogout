# wlogouts2

wlougout2 is a software to help wm users shut down.

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

- [x] shutdown
- [x] logout
- [x] reboot
- [x] hibernate
- [x] sleep
- [ ] optimize GUI
- [ ] systemd unit

[info](https://www.reddit.com/r/rust/comments/ec59eg/new_rust_library_to_shut_down_reboot_or_log_out/)


## Similar projects

- [wlogout](https://github.com/ArtsyMacaw/wlogout)
- [wleave](https://github.com/AMNatty/wleave)
- [pwrmenu](https://github.com/kamilernerd/pwrmenu)
- [rlogout](https://github.com/MarcusBoay/rlogout)

> This project is only used as a practice for learning rust and gtk
