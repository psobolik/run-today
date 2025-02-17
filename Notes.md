# Spaces

* In order to add a program with parameters, you must include the argument to the `add` command in double quotes,
  so it is passed to the program as a single parameter.
* If the command contains a space, or is in a folder with spaces, you must quote it with single quotes.

For example:

```
run-today add "'C:\Program Files\Something\something.exe'"
```

# Files

The application stores its files in a folder named `run-today` in standard directories, following the conventions of the
operating system the program is running on.

|                 | Windows                   | Linux                                          | macOS                               |
|-----------------|---------------------------|------------------------------------------------|-------------------------------------|
| `programs.json` | `{FOLDERID_LocalAppData}` | `\$XDG_CONFIG_HOME` or <br/>`$HOME/.config`    | `$HOME/Library/Application Support` |
| `last-run.json` | `{FOLDERID_LocalAppData}` | `\$XDG_DATA_HOME` or<br/> `$HOME/.local/share` | `$HOME/Library/Application Support` |
