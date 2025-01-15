# shlexy
Shell-parse input, output fields separated by a delimiter

Also handles quoted strings.

## Demo

```bash
$ echo 'a    b   "c d"     e' | shlexy
a,b,"c d",e
```

The output can be further processed with tools like `xsv`.
