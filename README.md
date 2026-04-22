# commas
Transform whitespace-separated input, either insert a delimiter or substitue a template string
with positional arguments.

## Features

- receive stdin, write stdout
- replace whitespace with a delimiter
- OR, substitute a template string with positional arguments
- handles quoted fields, and writes quoted fields

`commas` operates in two modes: template mode and delimiter mode. Template mode is activated with `-t`/`--template`, and delimiter mode is the default.

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

## The difference between `commas` and `awk`

`awk` is a full text-processing language; `commas` is a small shortcut.
There's obvious overlap. Both chew on whitespace-separated fields, but
three things push me to reach for `commas`:

**1. Shell-style quoted fields survive.** `awk` splits on whitespace (or
a regex, with `-F`) and has no notion of quoting. If a field is wrapped
in double quotes because it contains a space, `awk` treats the quote
character as just another character:

```bash
$ echo 'a "b c" d' | awk '{print $2}'
"b
$ echo 'a "b c" d' | commas -t '$2'
b c
```

To make `awk` respect shell-style quotes you have to roll your own parser
(or pull in `gawk`'s `FPAT`). `commas` gets it for free because it
delegates to `shlex`.

**2. Less syntax noise for the common case.** For straight field
reordering the template form is a little shorter than the equivalent
`awk` expression, and doesn't need you to think about `OFS`:

```bash
$ echo 'a b c' | awk '{print $2" "$3" "$1}'
b c a
$ echo 'a b c' | commas -t '$2 $3 $1'
b c a
```

**3. It's a one-trick pony, on purpose.** `awk` has `BEGIN`/`END`,
associative arrays, regex patterns, conditionals, and a full stdlib of
string/arithmetic functions. If you need any of that, use `awk`.
`commas` deliberately doesn't grow in that direction. When all you
want is "reformat these fields", the only syntax to remember is
`$1 $2 $3 …`.

## Releasing

Note to future-me: cutting a release is one command.

```bash
cargo install cargo-release  # one-time
cargo release patch --execute  # or `minor`, `major`, or a literal `X.Y.Z`
```

That invocation:

1. Bumps `version` in `Cargo.toml` and `Cargo.lock`.
2. Commits the bump.
3. Creates a git tag matching the new version, bare (e.g. `0.1.1`, no `v` prefix). Configured via `[package.metadata.release]` in `Cargo.toml`, which is what the release workflow's tag filter expects.
4. Pushes the commit and tag to `origin`.
5. Skips crates.io publish (not a library; `publish = false` in the same metadata block).

Drop `--execute` to see a dry run first.

### What CI does when the tag arrives

A push matching `[0-9]+.[0-9]+.[0-9]+` triggers `.github/workflows/release.yml`. It runs a single `build-release` matrix job across five targets:

| target                         | runner         | archive                                          |
| ------------------------------ | -------------- | ------------------------------------------------ |
| `x86_64-unknown-linux-musl`    | `ubuntu-22.04` | `commas-X.Y.Z-x86_64-unknown-linux-musl.tar.gz`  |
| `arm-unknown-linux-gnueabihf`  | `ubuntu-22.04` | `commas-X.Y.Z-arm-unknown-linux-gnueabihf.tar.gz`|
| `x86_64-apple-darwin`          | `macos-latest` | `commas-X.Y.Z-x86_64-apple-darwin.tar.gz`        |
| `x86_64-pc-windows-msvc`       | `windows-2022` | `commas-X.Y.Z-x86_64-pc-windows-msvc.zip`        |
| `i686-pc-windows-msvc`         | `windows-2022` | `commas-X.Y.Z-i686-pc-windows-msvc.zip`          |

`cross` is only installed for the ARM target (pre-built binary, pinned via `CROSS_VERSION`); the other four targets build natively on their runners.

Each matrix leg, on success, calls `softprops/action-gh-release` with the tag name. The first leg to finish creates the GitHub Release; subsequent legs attach their archive to the same release. `fail-fast: false` means if one target fails, the others still upload.

Action versions are SHA-pinned; dependabot (`.github/dependabot.yml`, `github-actions` ecosystem) bumps them on a daily schedule, so no manual maintenance is expected.

### If a release goes sideways

If a matrix leg fails and you want to retry cleanly:

```bash
git tag -d X.Y.Z                    # delete local tag
git push origin :refs/tags/X.Y.Z    # delete remote tag
gh release delete X.Y.Z             # if the release was already created
# fix the problem, then re-run `cargo release`
```

To re-run only the failed target without cutting a new tag, re-run the failed matrix leg from the GitHub Actions UI. If the leg failed *before* the upload step this just works; if it failed *during* upload and left a partial asset, delete that asset from the release page first (`softprops/action-gh-release` refuses to overwrite existing assets by default).
