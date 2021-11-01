# rew-down

rew-down is a software to help wm users shut down.

!! Not completed, it is not recommended to use it now !!

## BUILD

```bash
nix-zsh -p glib.dev pkg-config zlib cargo pango gdk-pixbuf gtk4 
cargo run
```

## TODO

- [x] shutdown   关机
- [ ] logout     注销
- [ ] reboot     重启
- [ ] hibernate  休眠 
- [ ] sleep      睡眠
- [ ] optimize GUI
- [ ] systemd unit
