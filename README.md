# commas
Transform whitespace-separated input, either insert a delimiter or substitue a template string
with positional arguments.

## Features

- receive stdin, write stdout
- replace whitespace with a delimiter
- OR, substitute a template string with positional arguments
- handles quoted fields, and writes quoted fields

## Examples: Template

This is a no-nonsense field substitution tool.

### Basic Test

```bash
$ echo 'a b c' | commas -t '$2 $3 $1'
b c a
```

Note that the fields start at 1, not 0. The only reason for this is to align with
bash so that you don't have to keep switching your mental model.

Quotes work:

```bash
$ echo 'a "b1 b2" c' | commas -t '$2 $3 $1'
b1 b2 c a
```


## Examples: Delimiter

### Basic Test

```bash
$ echo 'a b "c d" e' | commas
a,b,"c d",e
```

You can choose to lose quotes in the output

```bash
$ echo 'a b "c d" e' | commas -l
a,b,c d,e
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
