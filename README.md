# User Tracer
## Description
An example program that can install uprobes and print some information about the tasks that hit them.

## Usage
```
$ sudo ./target/release/user-tracer /lib/x86_64-linux-gnu/libreadline.so.8.1 0x21e50
Initializing...
Reading from queue...
comm=sqlite3, gid/uid=1000/1000, arg[0]=0x0055a1708c8a30, arg[1]=0x00000000000000, arg[2]=0x0055a170e59a80, arg[3]=0x00000000000c00
comm=sqlite3, gid/uid=1000/1000, arg[0]=0x0055a1708c8a30, arg[1]=0x00000000000000, arg[2]=0x0055a170e59a80, arg[3]=0x00000000000c00
comm=sqlite3, gid/uid=1000/1000, arg[0]=0x0055a1708c8a30, arg[1]=0x00000000000000, arg[2]=0x0055a170e59a80, arg[3]=0x00000000000c00
comm=sqlite3, gid/uid=1000/1000, arg[0]=0x0055a1708c8a30, arg[1]=0x00000000000000, arg[2]=0x0055a170e59a80, arg[3]=0x00000000000c00
comm=sqlite3, gid/uid=1000/1000, arg[0]=0x0055a1708c8a30, arg[1]=0x0055a170e59010, arg[2]=0x0000055a170e73, arg[3]=0x00000000000000
comm=sqlite3, gid/uid=1000/1000, arg[0]=0x0055a1708c8a30, arg[1]=0x0055a170e59010, arg[2]=0x0000055a170e73, arg[3]=0x00000000000000
comm=sqlite3, gid/uid=1000/1000, arg[0]=0x0055a1708c8a30, arg[1]=0x0055a170e59010, arg[2]=0x0000055a170e73, arg[3]=0x00000000000000
comm=sqlite3, gid/uid=1000/1000, arg[0]=0x0055a1708c8a30, arg[1]=0x0055a170e59010, arg[2]=0x0000055a170e73, arg[3]=0x00000000000000
comm=ftp, gid/uid=1000/1000, arg[0]=0x0055edb0324413, arg[1]=0x00000000000001, arg[2]=0x007ffdee8cb8f0, arg[3]=0x00000000000000
comm=ftp, gid/uid=1000/1000, arg[0]=0x0055edb0324413, arg[1]=0x00000000000001, arg[2]=0x007ffdee8cb8f0, arg[3]=0x00000000000000
comm=ftp, gid/uid=1000/1000, arg[0]=0x0055edb0324413, arg[1]=0x00000000000001, arg[2]=0x007ffdee8cb8f0, arg[3]=0x00000000000000
comm=ftp, gid/uid=1000/1000, arg[0]=0x0055edb0324413, arg[1]=0x00000000000001, arg[2]=0x007ffdee8cb8f0, arg[3]=0x00000000000000
```
