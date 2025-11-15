# LDR POC

POC for using Rust and PIGPIO to read LDR sensors on a RaspberryPI 4B

## PI Setup

You must be running 64 Bit image (Trixie) for these steps to work.

### PIGPIO

Instal PIGPIO from source, it no longer ships with the RPI image

```bash
sudo apt update
sudo apt install gcc make git
git clone https://github.com/joan2937/pigpio.git
cd pigpio
make
sudo make install
```

### Sysroot

To help with cross-compilation and to deal with all the libraries we introduce like PIGPIO we need to complete the following steps when ever we update the PI OS with new dependencies that our code makes use of.

1. Install pigpio on the Raspberry Pi

Debian Trixie does not include pigpio in apt repositories, so install from source.

```bash
sudo apt update
sudo apt install gcc make git

git clone https://github.com/joan2937/pigpio.git
cd pigpio
make
sudo make install
```

This installs:

- /usr/local/include/pigpio.h
- /usr/local/lib/libpigpio.so
- pigpiod
- pigs

2. Create a sysroot archive on the Raspberry Pi

We package the Pi filesystem (minus virtual mounts) so cross can access the correct headers and libraries.
This will take a long time to complete.

```bash
sudo tar -czf /home/operator/pi-rootfs.tar.gz \
    --exclude=/proc/* \
    --exclude=/sys/* \
    --exclude=/dev/* \
    --exclude=/run/* \
    --exclude=/tmp/* \
    /
```

3. Copy the sysroot to your macOS machine

From your Mac terminal:

```bash
scp operator@raspberrypi.local:/home/operator/pi-rootfs.tar.gz .
```

4. Extract sysroot on macOS

Choose a location, such as:

```bash
mkdir ~/Development/pi-sysroot
tar -xzf pi-rootfs.tar.gz -C ~/Development/pi-sysroot
```

5. Configure cross to use this sysroot

In the Rust project root, create a file named:

```bash
.crosstool.toml
```

Add:

```bash
[target.aarch64-unknown-linux-gnu]
sysroot = "/Users/operator/Development/pi-sysroot"
```

_(Change the macOS username path if needed.)_

6. Build the Rust project using cross

Clear previous builds:

```bash
cross clean
```

Then compile for Raspberry Pi 4:

```bash
cross build --target aarch64-unknown-linux-gnu --release
```

## Mac setup

Install Rust. WARNING do not install `Rust` via any other process than via `rustup` as the `cross` command will only work with `rustup` `Rust` installs.

Get rid of Rust if installed with Homebrew

```bash
brew uninstall rust
```

Install Rust via Rustup

```bash
curl https://sh.rustup.rs -sSf | sh
```

Install target (suitable for RPI 4 +)

```bash
rustup target add aarch64-unknown-linux-gnu
```

Install cross

```bash
cargo install cross --force --features docker-image
```

## Build project

You must have the sysroot inplace (see above) before building your project.

```bash
cross build --target aarch64-unknown-linux-gnu --release

```

## Copy program to the Raspberry PI

From your MacOS

```bash
scp target/aarch64-unknown-linux-gnu/release/<your-program> \
    operator@raspberrypi.local:~
```

_(Change the RaspberryPI username and host and path if needed.)_
