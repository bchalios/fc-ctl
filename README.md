# `fc-ctl` - An **unofficial** CLI tool for Firecracker microVMs

[Firecrakcer](https://github.com/firecracker-microvm/firecracker) is a Virtual Machine Monitor (VMM)
built on top of Linux KVM that allows running *microVMs*, i.e. normal KVM Virtual Machines with low
boot-times and small attack surface and low resource utilization.

Firecracker is configured through a RESTful API over a Unix Domain Socket (UDS). `fc-ctl` is a CLI tool
for configuring, running & snapshotting Firecracker microVMs, based on top of [`fclib`](https://github.com/bchalios/fclib).
It's main purpose is to use for testing.

The tool currently supports a subset of [Firecracker v1.4 API](https://github.com/firecracker-microvm/firecracker/blob/firecracker-v1.4/src/api_server/swagger/firecracker.yaml)
(mainly the things that are currently useful to me).

Feel free to open an issue or PR for adding support for additional missing or broken features.

## What `fc-ctl` is **NOT**

A Firecracker SDK. It does not manage the lifecycle of the VMM processes. It assumes that Firecracker VMM is 
launched and managed by other means. `fc-ctl` just speaks to a Firecracker process through its API server's
UDS to configure and launch the microVM, or perform post-boot operations such as snapshotting.

## Examples

```
# Configure a rootfs drive for microVM 
cargo run -- --api-sock /tmp/fc.sock drive add vda /path/to/rootfs.ext4 --is-root-device

# Configure a network device that uses tap device `tap0`.
# Use a rate-limiter for TX queue that allows 100MB/s
# and a rate-limiter for RX queue that allows 200MB/s
cargo run -- --api-sock /tmp/fc.sock net add eth0 tap0 --tx-bw 10000000 100 --rx-bw 20000000 100

# Configure the guest kernel and boot parameters
cargo run -- --api-sock /tmp/fc.sock kernel /home/ec2-user/workspace/fc-cli/share/vmlinux --boot-args "panic=1 reboot=k tsc=reliable earlyprintk=ttyS0 console=ttyS0 ipv6.disable=1 pci=off"

# Configure the microVM to have 1GB of memory and 4 vcpus
cargo run -- --api-sock /tmp/fc.sock machine-config config 1024 4

# Start the microVM
cargo run -- --api-sock /tmp/fc.sock microvm start
```

For a full list of the supported commands you can:

```
cargo run -- help
```

and for more info on a particular command:

```
cargo run -- drive --help
```

or subcommand:

```
cargo run -- drive add --help
```
