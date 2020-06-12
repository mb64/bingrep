# bingrep -- search for a fixed string in a large binary file

License: MIT

## Example

In `test_file.txt`:
```
This file contains the word "the" three times.
Where's the third?
```
Search for `the`:
```shell
$ bingrep the test_file.txt
Match from 19 to 22
Match from 29 to 32
Match from 55 to 58
```

## Why would you want to use this?

Well, suppose you had a program that wasn't yet in git, and which you
`rm`'d by accident.

With `bingrep`, it's easy! Just recall a short excerpt from your file, and
search your hard drive for it:

```shell
$ # First, how big is the hard drive
$ fdisk -l | awk '/sda3/ {print $4 * 512}'
94260690944
$ sudo bingrep -l `!!` 'my code excerpt' /dev/sda3
```

Then, after 30 minutes, when the only results are from `sudo` logging your
command to `/var/log/auth.log` (or journald, or wherever), you can go back and
rewrite your program.

## Features

- Handles binary queries and files
- Supports files as big as you can fit in your virtual address space (ie not
  big enough on 32-bit architectures)
- Might work on Windows
- Pretty progress bar courtesy of [`indicatif`](https://github.com/mitsuhiko/indicatif)
