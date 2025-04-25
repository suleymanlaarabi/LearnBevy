# Installation

## Ubuntu

**Installer les dépendance**
```shell
sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
```

**Si vous utiliser wayland**
```shell
sudo apt-get install libwayland-dev libxkbcommon-dev
```

**Installer LLD**
```shell
sudo apt-get install lld clang
```

**CraneLift**
```shell
rustup component add rustc-codegen-cranelift-preview --toolchain nightly
```

## Pour les autres
[setup](https://bevyengine.org/learn/quick-start/getting-started/setup/#rust-setup)
[fast compile time](https://bevyengine.org/learn/quick-start/getting-started/setup/#enable-fast-compiles-optional)
