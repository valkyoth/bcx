#!/usr/bin/env bash
set -euo pipefail

max_lines=500
failed=0

while IFS= read -r file; do
    lines="$(wc -l < "$file")"
    if [ "$lines" -gt "$max_lines" ]; then
        printf '%s has %s lines; maximum is %s\n' "$file" "$lines" "$max_lines"
        failed=1
    fi
done < <(find . -path './target' -prune -o -name '*.rs' -type f -print)

exit "$failed"
