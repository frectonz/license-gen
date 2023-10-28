# license-gen

A license generator made with rust.

The CLI needs an `email` and a `name` to generate the license file. It looks for them in a couple of places.

- Envoironment variable `USER`
- Git username form your `.gitconfig`
- Author field in `package.json`

## Run it with Nix

```bash
nix run github:frectonz/license-gen
```

## Install

```bash
cargo install license-gen
```

## Supported licenses

- Apache-2.0
- BSD-1-Clause
- BSD-2-Clause
- BSD-3-Clause
- ISC
- MIT
- UNLICENSE

## Demo

[![asciicast](https://asciinema.org/a/496158.svg)](https://asciinema.org/a/496158)
