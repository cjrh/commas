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

Finally, note that whitespace between the incoming values is all ignored,
except of course for what is quoted, which will be preserved:

```bash
$ echo 'a     "b1    b2"     c' | commas -t '$2 $3 $1'
b1    b2 c a
```

Here's a practical example. We can list the CPU cache layers:

```bash
$ lscpu | grep cache
L1d cache:                            256 KiB (8 instances)
L1i cache:                            256 KiB (8 instances)
L2 cache:                             4 MiB (8 instances)
L3 cache:                             8 MiB (2 instances)
```

`commas` can help to reformat this:

```bash
$ lscpu | grep cache | commas -t '$1 $3$4'
L1d 256KiB
L1i 256KiB
L2 4MiB
L3 8MiB
```

Going further, there is support for trimming characters off
each of the field values. In the above `lscpu` output, we
can get rid of the `:` after `cache:`, and the `(` preceding
the number of instances:

```bash
$ lscpu | grep cache | commas -t '$1 x $3$4 x $5' -s':('
L1d x 256KiB x 8
L1i x 256KiB x 8
L2 x 4MiB x 8
L3 x 8MiB x 2
```

## Examples: Delimiter

The delimiter mode cannot be used together with the template mode.
The delimiter mode is a shortcut to "just put commas between the fields"
although the delimiter is of course configurable.

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
have to use `sed` or `awk`, or even something bigger like Python
or Perl to handle this.

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
