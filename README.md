# Dev

First, install `just`, something similar to `make` and its respective `Makefile`.

```
cargo binstall just # Or cargo install just
```

Launch the game server

```
just server
```

And in another terminal, test using the client and/or cli to access the server.

```
just client
just cli 'args --or-flags' # Wrap in quotes if more than two pieces
```
