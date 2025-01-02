build-arm-prod:
	docker run --rm -it -v $(shell pwd)/backend:/project cartos-backend

build:
	docker build -f Dockerfile.rust.build.arm64 -t cartos-backend ./backend \
		&& make build-arm-prod
