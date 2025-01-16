#!/usr/bin/env bash
cargo build

CMD="./target/debug/commas"

printf "### Basic Test\n\n"
INPUT=$'echo \'a b "c d" e\' | '
CMDLINE_TEXT=$INPUT$CMD
CMDLINE_TEXT_CLEAN='$ '$INPUT"commas"
echo -e '```bash'"\n$CMDLINE_TEXT_CLEAN"
eval "$CMDLINE_TEXT"
echo '```'

printf "\n\n### Extra whitespace\n\n"
INPUT=$'echo \'a     b    "c d"        e\' | '
CMDLINE_TEXT=$INPUT$CMD
CMDLINE_TEXT_CLEAN='$ '$INPUT"commas"
echo -e '```bash'"\n$CMDLINE_TEXT_CLEAN"
eval "$CMDLINE_TEXT"
echo '```'

printf "\n\n### Pass through to \`xsv\`\n\n"
INPUT=$'echo \'a     b    "c d"        e\' | '
CMD="./target/debug/commas | xsv select 1,3"
CMDLINE_TEXT=$INPUT$CMD
CMDLINE_TEXT_CLEAN='$ '$INPUT"commas | xsv select 1,3"
echo -e '```bash'"\n$CMDLINE_TEXT_CLEAN"
eval "$CMDLINE_TEXT"
echo '```'

printf "\n\n### Reformat selected fields with \`xsv\`\n\n"
INPUT=$'echo \'a     b    "c d"        e\' | '
CMD="./target/debug/commas | xsv select 1,3,4 | xsv table"
CMDLINE_TEXT=$INPUT$CMD
CMDLINE_TEXT_CLEAN='$ '$INPUT"commas | xsv select 1,3,4 | xsv table"
echo -e '```bash'"\n$CMDLINE_TEXT_CLEAN"
eval "$CMDLINE_TEXT"
echo '```'
