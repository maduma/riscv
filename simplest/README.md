```
$ cargo build --release
$ qemu-system-riscv64 -nographic -machine shakti_c -bios ./target/riscv64imac-unknown-none-elf/release/simplest
```

With small code change, use virt machine
```
$ qemu-system-riscv64 -nographic -machine virt -bios ./target/riscv64imac-unknown-none-elf/release/simplest
```
