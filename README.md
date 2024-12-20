# Hitchcock

Hitchcock is a libre 3D experimentation tool.

## Development

### Clone

Clone this repository recursively.

```
git clone --recursive https://github.com/jroimartin/hitchcock.git
```

### Lint

Run the `cargo pedant` command (same as `cargo p`).

```
cargo pedant
```

## Examples

Examples are located in the </examples> folder. Use the following
command to run an example:

```
cargo run --example <name>
```

If `<name>` is not provided, then the examples are listed.

## Dependencies

- [GLFW 3](https://www.glfw.org/)
