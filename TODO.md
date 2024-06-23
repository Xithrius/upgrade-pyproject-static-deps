# TODO

## Rewrite everything (mostly)

Current implementation of deserialization `pyproject.toml` files to rust structures is very broken.

Comments, formatting, and indentation are all ignored when writing the new dependency versions.

Actions to complete the rewrite:

1. Handling the file

- [x] Make sure that the `pyproject.toml` *file* exists
- [x] Load it all into memory
- [x] Take metadata of the file to make sure that it was not modified since it was last read

2. Parsing sections/subsections

- [ ] Given an arbitrary attribute string (such as `project` or `tool.pdm.dev-dependencies`), make sure it exists within the file before giving information on where it is located
- [ ] When parsing, detect where the (sub)section ends
- [ ] Load in any attributes or subsections to the parent section

3. Parsing dependencies

- [ ] Make sure the list of dependencies exists, and is in proper format
- [ ] Parse the name, extras (if any) and version (put into semvar)
