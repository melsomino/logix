# Logix

Eazy to use log analyzer – good replacement for grep.

## Features

- Ultra fast log queries respect to indexing.
- Works with multiple log files and directories.
- Works with archived logs `.tar.gz`, `tar.xy`.
- Head and tail options out of the box.
- Context, before, after lines options.
- Fuzzy queries by full text search. 

## Installation
Clone this repo.
Run from repo root:

```shell
cargo install --path .
```

## Usage

Query single log file:

```shell
$ logix -p ./logs/foo.log failed
2025-03-27T12:15:38.105089Z ERROR foo::bar: foo/src/bar.rs:61: Calculation failed a < 3
2025-03-27T12:16:08.105731Z ERROR foo::bar: foo/src/bar.rs:20: Write to DB failed: invalid column name: baz
```

Query all log files in dir:

```shell
$ logix -p ./logs failed
./logs/foo1.log

2025-03-27T12:15:38.105089Z ERROR foo::bar: foo/src/bar.rs:61: Calculation failed a < 3
2025-03-27T12:16:08.105731Z ERROR foo::bar: foo/src/bar.rs:20: Write to DB failed: invalid column name: baz

./logs/foo2.log

2025-03-27T12:15:38.105089Z ERROR foo::bar: foo/src/bar.rs:61: Calculation failed a < 3
2025-03-27T12:16:08.105731Z ERROR foo::bar: foo/src/bar.rs:20: Write to DB failed: invalid column name: baz
```

Query for all words:

```shell
$ logix -p ./logs/foo.log failed calc
2025-03-27T12:15:38.105089Z ERROR foo::bar: foo/src/bar.rs:61: Calculation failed a < 3

$ logix -p ./logs/foo.log failed db
2025-03-27T12:16:08.105731Z ERROR foo::bar: foo/src/bar.rs:20: Write to DB failed: invalid column name: baz
```

Query alternatives:

```shell
$ logix -p ./logs/foo.log 'failed|invalid'
2025-03-27T12:15:38.105089Z ERROR foo::bar: foo/src/bar.rs:61: Calculation failed a < 3
2025-03-27T12:16:08.105731Z ERROR foo::bar: foo/src/bar.rs:20: Invalid component index 3 (should be less than 2)
```

Query by whole words (by default it uses prefix):

```shell
$ logix -p ./logs/foo.log 'fail' -w
2025-03-27T12:15:38.105089Z ERROR foo::bar: foo/src/bar.rs:61: Critical fail: no room on disk
```

Query with important word order:

```shell
$ logix -p ./logs/foo.log 'db failed' -o
2025-03-27T12:16:08.105731Z ERROR foo::bar: foo/src/bar.rs:20: Write to DB failed: invalid column name: baz
```
this query skips line like:
```shell
2025-03-27T12:16:08.105731Z ERROR foo::bar: foo/src/bar.rs:20: Failed to connect to DB
```

## Log file processing

If you specify path to file, logix will use specified file.

If specified file is an archive (*.tar.gz, *.tar.xz) then logix will extract all *.log files from this archive 
and will delete archive.

To optimize query speed logix creates index file with name "source-file-name.ix". For example, it creates 
file "foo.log.ix" for file "foo.log".

If specified path is a directory, logix will recursively scan directory for files *.log, *.tar.gz, *.tar.xz and 
will process each found file as described above.

