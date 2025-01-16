# commas
Transform whitespace-separated input into comma-separated output.

## Features

- receive stdin, write stdout
- handles quoted fields, and writes quoted fields

## Demo

```bash
$ echo 'a    b   "c d"     e' | shlexy
a,b,"c d",e
```

The output can be further processed with tools like `xsv`.
