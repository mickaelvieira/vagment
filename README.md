# Vagment

A simple command line tool to manage Vagrant Virtual Machines.

## Installation

```
$ curl -s -L -o - https://raw.githubusercontent.com/mickaelvieira/vagment/master/bin/install | bash
$ sudo mv vagment /usr/local/bin/vagment
$ vagment --version
```

## Documentation

List all the virtual machines

```
$ vagment list
```

Dump the `Vagrantfile`

```
$ vagment dump <Machine Number>
```

Execute a vagrant command against a given machine

```
$ vagment <Vagrant command> <Machine Number>
```
