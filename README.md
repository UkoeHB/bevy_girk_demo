# Girk Demo

Demo for [bevy_girk](https://github.com/UkoeHB/bevy_girk).


### Full Usage

Run the backend:
```
cargo run -p backend
```

Run `client 0`:
```
cargo build -p game_client
&& cargo build -p game_instance
&& cargo run -p user_client -- --id 0
```

Run `client 1`:
```
cargo run -p user_client -- --id 1
```


### Playtest

Run a local multiplayer game from the command line:
```
cargo build -p game_client
&& cargo build -p game_instance
&& cargo run -p playtest -- --clients 2
```


### Major TODOs

- Update `bevy_lunex` to a version that works (hit test depth detection is broken).
- Get WASM client apps working. Right now only the user client works. Needs a `renet` WASM transport, and needs a workflow for launching client apps in new tabs.
- Figure out config story. Right now configs are scattered in various rust files. Config assets? Ergonomic config access and customization?
- Feature demos:
    - Ping tracker.
    - Input status tracking for client-side prediction.
    - Game replay dev tooling.
- Deployment to production?
- Mobile?
