- [ ] option `-i` to ignore case, supercedes envvar
- [ ] settable IGNORE_CASE envvar value read in, 1 for on, 0 for off
- tests for ignore_case option
  - [ ] test that sets IGNORE_CASE=1, no oarg then run() => insensitive search
  - [ ] test that sets IGNORE_CASE=0, no oarg then run() => sensitive search
  - [ ] test that sets IGNORE_CASE=1, -i then run() => insensitive search
  - [ ] test that sets IGNORE_CASE=0, -i then run() => insensitive search