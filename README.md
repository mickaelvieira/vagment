# Vagment

A simple command line tool to manage your [Vagrant](https://www.vagrantup.com) Virtual Machines.

## Installation

##### Dependencies
- [rust](https://www.rust-lang.org/)
- [cargo](https://github.com/rust-lang/cargo)
- [curl](https://curl.haxx.se/docs/)

```sh
$ curl -s -L -o - https://raw.githubusercontent.com/mickaelvieira/vagment/master/bin/install | bash
$ sudo mv vagment /usr/local/bin/vagment
$ vagment --version
```

## Documentation

### Motivation

[Vagrant](https://www.vagrantup.com) provides a great client to interact with
environments however I always find myself typing the same commands and
I wanted a tool to be a bit more productive. Or maybe, I simply needed a side project to
start learning [Rust](https://www.rust-lang.org/) :).

This tool aims to bring useful functionalities when dealing with one or multiple
virtual machines such as:
- Quickly edit the Vagrantfile
- Quickly `cat` the Vagrantfile
- Autostart the machine before `ssh`ing into it
- Boot up or shutdown all machines available
- Run common commands `up`, `halt`, `reload`, etc... without having to `cd` into the directory

Even though you can do something like `vagrant up [name|id]`, I usually don't really set
the virtual machine's name (call me lazy) and honestly who seriously remembers those IDs.

I personally prefer having to type or remember a `machine number` instead of
an ID to execute a command.

### Basic usage

```sh
$ vagment list

  Number   |    Name    |   State    |    Path
---------- | ---------- | ---------- | ----------
    1      |  default   |  running   | /path/to/vm1
    2      |  default   |  poweroff  | /path/to/vm2
    3      |  default   |  running   | /path/to/vm3

$ vagment up 2
$ vagment halt 1
```

***NOTE***: If you don't specify the `machine number` and you have only one virtual machine, `vagment` will use it otherwise you will be prompted for the `machine number`.

***NOTE***: if you execute `vagment ssh` and the machine is not running, `vagment` will start it up for you.

### Commands

Display help

```
$ vagment help
```

List all the virtual machines

```sh
$ vagment list
```

Boot up all stopped virtual machines

```sh
$ vagment bootup
```

Halt all running virtual machines

```sh
$ vagment shutdown
```

Refresh [global status](https://www.vagrantup.com/docs/cli/global-status.html) command's cache

```sh
$ vagment refresh
```

Dump the `Vagrantfile`

```sh
$ vagment dump <Machine Number>
```

Open the `Vagrantfile` with your favourite `$EDITOR`

```sh
$ vagment edit <Machine Number>
```

Execute one of the following `vagrant` commands:
- `up`
- `ssh`
- `halt`
- `reload`
- `resume`
- `status`
- `destroy`
- `suspend`

```sh
$ vagment <Vagrant Command> <Machine Number>
```

Or simply

```sh
$ vagment <Vagrant Command>
```

## License

The MIT License (MIT). Please see [License File](https://github.com/mickaelvieira/vagment/tree/master/LICENSE.md) for more information.
