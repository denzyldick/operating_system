```bash
cargo rustc -- -C link-arg=-nostartfiles
```

```bash
qemu-system-x86_64 -drive format=raw,file=target/x84_64/debug/bootimage-operating_system.bin
```