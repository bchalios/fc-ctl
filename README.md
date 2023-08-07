# `fc-ctl` - An **unofficial** CLI tool for Firecracker microVMs

[`Firecrakcer`](https://github.com/firecracker-microvm/firecracker) is a Virtual Machine Monitor (VMM)
built on top of Linux KVM that allows running *microVMs*, i.e. normal KVM Virtual Machines with low
boot-times and small attack surface and low resource utilization.

`Firecracker` is configured through a RESTful API over a Unix Domain Socket (UDS). `fc-ctl` is a CLI tool
for configuring, running & snapshotting Firecracker microVMs, based on top of [`fclib`](https://github.com/bchalios/fclib).
It's main purpose is to use for testing.

The tool currently supports a subset of [Firecracker v1.4 API](https://github.com/firecracker-microvm/firecracker/blob/firecracker-v1.4/src/api_server/swagger/firecracker.yaml)
(mainly the things that are currently useful to me).

Feel free to open an issue or PR for adding support for additional missing or broken features.
