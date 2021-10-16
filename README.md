```bash
cargo rustc -- -C link-arg=-nostartfiles
```

```bash
qemu-system-x86_64 -drive format=raw,file=target/x84_64/debug/bootimage-operating_system.bin

####preview
```![Screenshot from 2021-10-16 14-19-28](https://user-images.githubusercontent.com/2477646/137583712-44261ea8-15b4-4518-bf90-21589bcfc998.png)
