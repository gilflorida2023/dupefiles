
# dupefiles

Finds all duplicate files in a specified sub-directory tree specified on command-line and writes the results to a csv file.

Note: dupefiles skips hidden files/directories and zero byte files.

```bash
$./dupefiles
Usage: ./dupefiles <directory>
Finds all duplicate files in a specified sub-directory tree specified on command-line.
```

```bash
$ dupefiles ~/Downloads/ > /tmp/out.csv
Elapsed time: 2s 994ms
$ cat /tmp/out.csv
DUPE1.NAME,DUPE1.SIZE,DUPE2.NAME,DUPE2.SIZE
"/home/linux/Downloads/dupe.jpg",85448,"/home/linux/Downloads/Chung_1_1000.jpg",85448
$ 
```

Compile and run with debug feature enabled. Prints all files encountered.

```bash
#!/usr/bin/env bash
cargo build --features debug
cargo run --features debug  -- $@
```
