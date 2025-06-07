# Portfolio

My little slice of the internet :)

## Requirements

- `todo!()`
- `tmux`

## Scripts

- `dev` runs dev environment: server + vite + bacon
  - The `bacon` pane includes hotkeys:
    - `c` **c**lippy (default)
    - `w` wasm **d**ev compilation
    - `b` wasm release **b**uild
- `build` builds client for release
- `clean:dev` deletes all dev build binaries (debug WASM, primarily)
- `clean:release` deletes all built release files (`dist/`, `release/`, etc)
