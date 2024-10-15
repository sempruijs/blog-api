# User space


A website for manging users.

## Setup devshell

#### With nix

To enter a devshell, enable nix flakes and run:

```shell
  nix develop
```

To view the website on your machine, run:

```shell
ORIGIN=http://localhost:3000 nix run .#site
```

### Without nix

For frontend development:

Make sure to install npm on your machine and read [the svelte init readme](./site/README.md)

For backend development:

Make sure you install cargo. Rust analyser is recommendend as LSP.

## License

BSD-2
