# User Tracer

## Description
An example program that can install uprobes and print some information about the tasks that hit them.

## Usage

### Example Tracing libreadline/readline
```
$ sudo ./target/release/user-tracer /lib/x86_64-linux-gnu/libreadline.so.8.1 readline --arg cstr
Initializing...
Reading from queue...
comm=ftp, gid/uid=1000/1000, args=["ftp> "]
```

### Example Tracing libc/write
```
$ sudo ./target/release/user-tracer /lib/x86_64-linux-gnu/libc.so.6 write --arg num --arg cstr --arg num
Initializing...
Reading from queue...
comm=write-test, gid/uid=1000/1000, args=[3, "hello world", 11]
```
