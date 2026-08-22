#!/usr/bin/env python3
"""Unwrap `tokio::task::spawn_blocking(move || BODY).await.unwrap()` back to
just `BODY`, in-place, for the given file(s).

The rust-axum openapi-generator template offloads every request body
validation and response serialization onto Tokio's blocking thread pool,
even though both are fast, non-blocking, synchronous CPU work. Under load
this makes every request pay for a real OS-level thread wake (twice), which
dominates CPU time for otherwise-cheap requests. Neither is I/O or long
enough to justify a blocking-pool trip, so the fix is to just call the
closure body inline in the same async task.

A simple regex cannot express this correctly because the closure body
itself contains nested, unbalanced-looking parentheses (e.g. nested
`.map_err(|e| { ... })` calls). This walks the source character-by-character
to find the paren that actually balances `spawn_blocking(`'s opening paren.
"""

from __future__ import annotations

import re
import sys

# The generator's raw (pre-rustfmt) output has irregular whitespace, and the
# closure body may start on the following line, so match structurally rather
# than on a fixed string.
HEAD_RE = re.compile(r"tokio::task::spawn_blocking\(\s*move\s*\|\|\s*")


def find_matching_paren(text: str, open_index: int) -> int:
    """Return the index of the ')' that closes the '(' at open_index."""
    assert text[open_index] == "("
    depth = 0
    for i in range(open_index, len(text)):
        if text[i] == "(":
            depth += 1
        elif text[i] == ")":
            depth -= 1
            if depth == 0:
                return i
    raise ValueError("unbalanced parentheses")


def unwrap(text: str) -> tuple[str, int]:
    replacements = 0
    out = []
    pos = 0
    while True:
        head_match = HEAD_RE.search(text, pos)
        if head_match is None:
            out.append(text[pos:])
            break
        start = head_match.start()
        open_paren = start + len("tokio::task::spawn_blocking")
        close_paren = find_matching_paren(text, open_paren)
        body = text[head_match.end() : close_paren]

        # After the closing paren, expect (allowing whitespace/newlines):
        # .await\n    .unwrap()  — optionally followed by `?`.
        tail_match = re.match(
            r"\s*\.await\s*\.unwrap\(\)(\??)",
            text[close_paren + 1 :],
        )
        if tail_match is None:
            raise ValueError(
                f"spawn_blocking at offset {start} not followed by "
                ".await.unwrap() as expected"
            )
        trailing_question_mark = tail_match.group(1)

        out.append(text[pos:start])
        out.append(body)
        out.append(trailing_question_mark)
        pos = close_paren + 1 + tail_match.end()
        replacements += 1

    return "".join(out), replacements


def main() -> int:
    if len(sys.argv) < 2:
        print("usage: unwrap_spawn_blocking.py <file>...", file=sys.stderr)
        return 2
    for path in sys.argv[1:]:
        with open(path, "r") as f:
            text = f.read()
        new_text, count = unwrap(text)
        if count:
            with open(path, "w") as f:
                f.write(new_text)
        print(f"unwrap_spawn_blocking: {path}: {count} occurrence(s) unwrapped", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
