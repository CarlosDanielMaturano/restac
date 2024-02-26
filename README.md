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

<p><strong>NOTE:</strong> some expressions( such as negatives numbers and multiplying parenthesis ) are not yet supported: </p>

```bash
    expression>
    -5 + 3
    thread 'main' panicked at src/evaluator.rs:21:34:
    Cannot pop out of stack
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    (4 + 3)(4 - 3)
    thread 'main' panicked at src/evaluator.rs:21:34:
    Cannot pop out of stack
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

# Contributing

## <p>Open an issue describing the feature/bug fix and make a PR with your code, i'will check and merge</p>

---

### Testing

<p>Unit tests are included, you can check if the code is properly working by running: </p>

```bash
    cargo test
```

<p>Just remember to add more unit tests if you add something to the code</p>

## Todo

    [] Bug fix of negative numbers and multiplying parenthesis
    [] Add some more advance functions, such as potentiation and trigonometric
