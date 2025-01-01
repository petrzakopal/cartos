# Cartos - NFC Card Reader project

_currently in development_


_documentation will be added later_

Also, the code is not finished at all. Currently rewriting the one file code from my secret [Gist](https://gist.github.com/petrzakopal/1ac14b9599f5657b5d2d78d7e89441d6).



# Run websocket testing utility on Linux

[websocat](https://github.com/vi/websocat)

Using following command.

```sh
websocat "ws://0.0.0.0:4000/ws"
```

When `websocat` is not installed or not in path download the prebuilt binaries
and run with the same argument.

```sh
./websocat.x86_64-unknown-linux-musl "ws://0.0.0.0:4000/ws"
```
