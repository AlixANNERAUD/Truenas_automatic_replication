<h1 align=center>💾TrueNAS automatic replication💽</h1>

## 🚀 Description

This rust program is a simple tool to automatically replicate a dataset from a TrueNAS server to a linux machine (Ubuntu). It uses the TrueNAS REST API to get the list of datasets and send replication requests to the TrueNAS server.

## 🛠️ Compilation

The program can be compiled using the following command:

```sh
cargo build --release
```

Since `zpool` is used to mount and unmount the local dataset, the program must be run as `root` or you should add the following line to your `/etc/sudoers` file:

```sh
your_user ALL=(ALL) NOPASSWD: /usr/sbin/zpool
```

## 🏃‍♂️ Usage

The program is meant to be run as a systemd service. It reads its configuration from environment variables. The following environment variables are required:

- `TRUENAS_SCALE_HOST`: The hostname of the TrueNAS server (e.g. `truenas.local`)
- `TRUENAS_SCALE_TOKEN`: The token to use for the TrueNAS REST API.
- `TRUENAS_SCALE_TASKS`: A `:` separated list of tasks name to run. Each task is a `:` separated list of the source dataset and the destination path (e.g. `task1:task2`).
- `LOCAL_DATASETS`: The local dataset to mount and unmount for each task.
- `LOCAL_DISKS`: The local disks to shutdown after the replication is done.
- `RUST_LOG`: The log level to use for the program (e.g. `info`).

The program can be run using the following command:

```bash
TRUENAS_SCALE_HOST=truenas.local \
TRUENAS_SCALE_TOKEN=token \
TRUENAS_SCALE_TASKS=task1:task2 \
LOCAL_DATASETS=dataset1:dataset2 \
LOCAL_DISKS=/dev/sda:/dev/sdb \
RUST_LOG=info \
./target/release/truenas_replication
```

The program will mount the local dataset, replicate the dataset from the TrueNAS server and then unmount the local dataset. It will then shutdown the local disks.

## ℹ️ About

This program is written by [Alix ANNERAUD](https://alix.anneraud.fr) and is distributed under the MIT license.