# Restac

## A rust math interpreter that use reverse polish notation to evaluate expressions

![Static Badge](https://img.shields.io/badge/cargo-1.74.1%20-blue)
![Static Badge](https://img.shields.io/badge/LICENSE-MIT-green)

# Usage

```bash
    cargo run
    expression>
    1 + 3 - 5
    -1
    1 * (0 - 5)
    -5
    (9 + 1) * (4 * 20 / 2)
    400
```

<p><strong>NOTE:</strong> some expressions without parenthesis are not yet supported: </p>

```bash
    expression>
```
