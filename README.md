# micrus
A 64-bit modular operating system microkernel written in rust, which allows a user to select modules they'd like. the micrus microkernel only contains the very minimum functionality needed to boot:
1. basic keyboard input
1. video graphics array output
1. interrupt descriptor table
1. panic handler
1. simple memory management
1. system timers

## Getting Started
### Prerequisites
1. Download VirtualBox
1. Ensure you have a means to run `dd` (for example, via git bash on windows)
1. Ensure you're on the nightly toolchain for rust by running `rustup toolchain nightly`
1. Optionally, set your default rust toolchain to nightly by running `rustup default nightly`

### Building micrus
1. Download this repo
1. Either use the [build script](./build/build.ps1) or run the steps within it to build the kernel

### Running micrus
Upon booting micrus, the user will be prompted to select which modules to include. Currently, the planned modules are:
1. filesystem
1. network stack
1. shell
1. package manager
1. graphical desktop