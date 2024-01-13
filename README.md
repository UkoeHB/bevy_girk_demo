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

There is a playtest binary for running a game from the command line.

Here is the command for a 2-player game:
```
cargo build -p game_client
&& cargo build -p game_instance
&& cargo run -p playtest -- --clients 2
```
