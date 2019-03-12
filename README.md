# hash-data

A library and command line tool for identifying hashes.

### Examples

Using the library:

```rust
assert_eq!(hash_data::parse("$1$42bad211$ums.eDtzK/1711rUkRsd31"), vec!["MD5(Unix)"])
```

On the command line:

```sh
$ hash-data '$1$42bad211$ums.eDtzK/1711rUkRsd31'
MD5(Unix)
```

License: MIT
