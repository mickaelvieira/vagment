# Vagment

A simple command line tool to manage [Vagrant](https://www.vagrantup.com) Virtual Machines.

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

[Vagrant](https://www.vagrantup.com) provides a great client to interact with
virtual machines however I always find myself typing the same commands and
I wanted a tool to be more productive. (or maybe I simply needed a side project to learn [Rust](https://www.rust-lang.org/))

Even though you can do something like `vagrant up [name|id]`, I usually don't set
the virtual machine's name (call me lazy) and honestly who seriously remembers those IDs.

I personally find much easier to type a `number machine` instead of an ID to execute
a command against an existing machine.

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

***NOTE***: If you have only one virtual machine, you don't even need to type the `machine number`.

### Commands

Display `usage` screen

```
$ vagment help
```

List all the virtual machines

```sh
$ vagment list
```

Halt all the virtual machines

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

Open the `Vagrantfile` in your favourite `$EDITOR`

```sh
$ vagment edit <Machine Number>
```

Execute one of the following `vagrant` commands against a given machine:
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
***NOTE***: If you have more than one virtual machine, you will be prompted for the machine number.

***NOTE***: if you execute `vagment ssh` when the machine is not running, `vagment` will start up the machine for you.

## License

The MIT License (MIT). Please see [License File](https://github.com/mickaelvieira/vagment/tree/master/LICENSE.md) for more information.
