# install 

```
$ curl https://sh.rustup.rs -sSf | sh
info: downloading installer

Welcome to Rust!

This will download and install the official compiler for the Rust
programming language, and its package manager, Cargo.

Rustup metadata and toolchains will be installed into the Rustup
home directory, located at:

  /home/loic/.rustup

This can be modified with the RUSTUP_HOME environment variable.

The Cargo home directory located at:

  /home/loic/.cargo

This can be modified with the CARGO_HOME environment variable.

The cargo, rustc, rustup and other commands will be added to
Cargo's bin directory, located at:

  /home/loic/.cargo/bin

This path will then be added to your PATH environment variable by
modifying the profile files located at:

  /home/loic/.profile
  /home/loic/.bashrc

You can uninstall at any time with rustup self uninstall and
these changes will be reverted.

Current installation options:


   default host triple: x86_64-unknown-linux-gnu
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation

info: profile set to 'default'
info: default host triple is x86_64-unknown-linux-gnu
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
info: latest update on 2021-02-11, rust version 1.50.0 (cb75ad5db 2021-02-10)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
 14.7 MiB /  14.7 MiB (100 %)   4.6 MiB/s in  3s ETA:  0s
info: downloading component 'rust-std'
 24.5 MiB /  24.5 MiB (100 %)   6.0 MiB/s in  4s ETA:  0s
info: downloading component 'rustc'
 60.3 MiB /  60.3 MiB (100 %)   6.1 MiB/s in 10s ETA:  0s
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: using up to 500.0 MiB of RAM to unpack components
info: installing component 'clippy'
info: installing component 'rust-docs'
 14.7 MiB /  14.7 MiB (100 %)   7.9 MiB/s in  1s ETA:  0s
info: installing component 'rust-std'
 24.5 MiB /  24.5 MiB (100 %)   9.5 MiB/s in  3s ETA:  0s
info: installing component 'rustc'
 60.3 MiB /  60.3 MiB (100 %)  10.7 MiB/s in  5s ETA:  0s
info: installing component 'rustfmt'
info: default toolchain set to 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu installed - rustc 1.50.0 (cb75ad5db 2021-02-10)


Rust is installed now. Great!

To get started you need Cargo's bin directory ($HOME/.cargo/bin) in your PATH
environment variable. Next time you log in this will be done
automatically.

To configure your current shell, run:
source $HOME/.cargo/env

```

# init
creation du projet
```
cargo new fact
cd fact
```
pour tester les perfs avec [criterion](https://docs.rs/criterion/0.3.5/criterion/) ajouter dans cargo.toml:
```
[dev-dependencies]
criterion = "0.3"

[[bench]]
name = "my_benchmark"
harness = false
```
# dev
ajouter les fonctions à tester dans my_benchmark.rs et utiliser les macros criterion_group! et criterion_main!

# exec
```
cargo build --release
./target/release/fact
```

pour mesurer les perfs
```
cargo bench
```

