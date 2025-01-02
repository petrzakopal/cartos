# Cartos - NFC Card Reader project

_currently in development_


_documentation will be added later_

Also, the code is not finished at all. Currently rewriting the one file code from my secret [Gist](https://gist.github.com/petrzakopal/1ac14b9599f5657b5d2d78d7e89441d6).


# Build
> [!TIP]
> Easiest way to build the app is run `make build`
> in the project root.

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

# API Requests for Bruno

For testing the api the [Bruno](https://github.com/usebruno/bruno) open source tool for testing
the endpoints was used. The files of bruno requests can be found in the folder [bruno_api_test](./bruno_api_test).

# Cross compilation of the Rust application in Docker

Either create directory and file in the project root `./cargo/Config.toml` and insert following values.

```toml
[build]
target = "aarch64-unknown-linux-gnu"

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

Or it can be defined in-line using following `RUST_FLAGS`.

```sh
RUSTFLAGS="-C linker=aarch64-linux-gnu-gcc" cargo build --release --target aarch64-unknown-linux-gnu
```

Also for the build the [Dockerfile](Dockerfile.rust.build.arm64) can be used.

The container can be build using following command.

```sh
docker build -f Dockerfile.rust.build.arm64 -t cartos-backend ./backend
```

## Manually in `bash`
One approach is to log in to the bash in the docker container and perform the build process manually.

> [!NOTE]  
> This approach is not suitable for CI/CD or any automated approach.

And the image then can be run manually in the project root using `docker run --rm -it -v $(pwd):/project rust-cross-aarch64 bash`.

Then the build process for the ARM64 architecture can easily run using `Makefile` when following is specified.

```sh
arm64:
	RUSTFLAGS="-C linker=aarch64-linux-gnu-gcc" cargo build --release --target aarch64-unknown-linux-gnu
```

Then run the build using `make arm64`.

The project is build to the `target/<architecture>/` directory.

## Automatically

> [!NOTE]  
> This approach is suitable for CI/CD or any automated approach.

After building the docker container, just run `docker run --rm -it -v $(pwd):/project rust-cross-aarch64` and the
project is automatically build to the `target/<architecture>/` directory.

# When using docker
> [!CAUTION]
> Buildrun does not work when cross-compiling. Could not
> figure how to make it work.

The file [Dockerfile.rust.buildrun.arm64](Dockerfile.rust.buildrun.arm64) can be used to build and then run the application.

To be able to use HW devices such as GPIO or NFC reader, the devices must be passed to the `docker build` command.

```sh
sudo docker run --device=/dev/gpiochip0 <image-name>
```

To be able to use GPIO and NFC using pcsd use probably the following command.

```sh
sudo docker run --device=/dev/gpiochip0 -v /var/run/pcscd/pcscd.comm:/var/run/pcscd/pcscd.comm gpio-rust-test
```

> [!NOTE]  
> When trying only to build, use the []().

# Run the backend as a service in Linux

```sh
sudo vi /etc/systemd/system/cartos.service
```

Insert contents from [service/cartos.service](service/cartos.service).

```sh
sudo systemctl daemon-reload
```

```sh
sudo systemctl enable cartos.service
```

```sh
sudo systemctl start cartos.service
```

```sh
sudo systemctl status cartos.service
```

# When running on the Orange Pi 3 W

> [!WARNING]  
> The USB ACS Reader must be reconnected after the system reboots and the service/app 
> running cartos must be also restarted.

After the board reboots, there is a need to unplug and plug the USB of the NFC reader, then run

```sh
sudo systemctl restart cartos
```

so the USB reader can be read by the backend application.


> [!NOTE]  
> This approach is not suitable but could not find how to make it automatically.

# Services

The service for backend can be installed with the usage of files in [service](.service).

## `install_requirements.sh`

Installs the requirements for the pcsc and other required configs.


## `install.sh`

Installs the service to the daemon.

## `logs.sh`

Opens logs of the `cartos.service`.
