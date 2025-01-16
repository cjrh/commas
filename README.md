# commas
Transform whitespace-separated input into comma-separated output.

## Features

- receive stdin, write stdout
- handles quoted fields, and writes quoted fields

## Demo

### Basic Test

```bash
$ echo 'a b "c d" e' | commas
a,b,"c d",e
```

### Extra whitespace

Note that `tr` does not handle this. Without `commas` you would
have to use `sed` or `awk` to handle this.

```bash
$ echo 'a     b    "c d"        e' | commas
a,b,"c d",e
```


### Pass through to `xsv`

The `xsv` tool can do a lot, but it needs comma-separated input.
Piping data to `xsv` was the primary motivation for `commas`.

Here's an example of field selection with `xsv`:

```bash
$ echo 'a     b    "c d"        e' | commas | xsv select 1,3
a,c d
```

### Reformat selected fields with `xsv`

```bash
$ echo 'a     b    "c d"        e' | commas | xsv select 1,3,4 | xsv fmt -t '|'
a|c d|e
```
