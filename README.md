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

Halt all the virtual machines

```
$ vagment shutdown
```

Refresh [global status](https://www.vagrantup.com/docs/cli/global-status.html) command's cache

```
$ vagment refresh
```

Dump the `Vagrantfile`

```
$ vagment dump <Machine Number>
```

Open the `Vagrantfile` in your favourite `$EDITOR`

```
$ vagment edit <Machine Number>
```

Execute one of the following vagrant commands against a given machine
- up
- ssh
- halt
- reload
- resume
- status
- destroy
- suspend

```
$ vagment <Vagrant command> <Machine Number>
```

## License

The MIT License (MIT). Please see [License File](https://github.com/mickaelvieira/vagment/tree/master/LICENSE.md) for more information.
