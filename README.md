# Girk Demo

Demo for [bevy_girk](https://github.com/UkoeHB/bevy_girk).


### Full Usage

Run the backend:
```
cargo build -p game_instance && \
cargo run -p backend
```

Run `client 0`:
```
cargo run -p client -- --id 0
```

Run `client 1`:
```
cargo run -p client -- --id 1
```


### Playtest

Run a local multiplayer game from the command line:
```
cargo build -p game_client && \
cargo build -p game_instance && \
cargo run -p playtest -- --clients 2
```


### Major TODOs

- Figure out config story. Right now configs are scattered in various rust files. Config assets? Ergonomic config access and customization?
- Add mobile support.
    - Requires tying UI layout to device type (mobile landscape/portrait and native).
