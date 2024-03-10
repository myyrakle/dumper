# trash_dumper
![](https://img.shields.io/badge/language-Rust-red) ![](https://img.shields.io/badge/version-1.0.1-brightgreen) [![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/myyrakle/trash_dumper/blob/master/LICENSE)

simple trash file dumper

## Install 

```
cargo install trash_dumper
```

## How to use

It's simple to use. 
For example, the command below creates three trash files of 10 MB in size.
```
trash_dumper dump -s 10m -c 3
```

1. 5 = 5 byte
2. 5k = 5 kb
3. 5m = 5 mb
4. 5g = 5 gb

Cleaning your trash is also simple. 
When you use the clean command, all files with the .trashfile extension are deleted.
```
trash_dumper clean
```