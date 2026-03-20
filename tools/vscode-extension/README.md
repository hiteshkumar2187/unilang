# UniLang for Visual Studio Code

Language support for **UniLang**, a hybrid programming language that combines Python and Java syntax in `.uniL` files.

## Features

- Syntax highlighting for UniLang's dual Python + Java grammar
- Code snippets for both Python-style and Java-style constructs
- Bracket matching, auto-closing pairs, and smart indentation
- Comment toggling (line: `//` and `#`, block: `/* */`)
- Folding support (indentation-based and brace-based)

## Supported Syntax

UniLang supports a unified syntax that includes:

- **Python constructs**: `def`, `class`, `import`, `with`, `async/await`, decorators, f-strings, triple-quoted strings, type annotations
- **Java constructs**: access modifiers, static/final, interfaces, enums, generics, try/catch, switch/case
- **Shared constructs**: if/else, for/while loops, comments, operators

## Snippets

| Prefix       | Description                          |
|------------- |--------------------------------------|
| `defpy`      | Python-style function                |
| `defjava`    | Java-style method                    |
| `classpy`    | Python-style class                   |
| `classjava`  | Java-style class                     |
| `ifpy`       | Python if/elif/else                  |
| `ifjava`     | Java if/else                         |
| `for`        | Python for loop                      |
| `forj`       | Java for loop                        |
| `try`        | Python try/except                    |
| `trycatch`   | Java try/catch/finally               |
| `main`       | Java main method                     |
| `import`     | Import statement                     |
| `fromimport` | From-import statement                |
| `fstring`    | F-string with interpolation          |
| `thread`     | Thread pool executor pattern         |
| `mlmodel`    | ML model class skeleton              |

## Requirements

VS Code 1.75.0 or later.
