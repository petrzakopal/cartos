build-backend-arm-prod-exe:
	docker run --rm -it -v $(shell pwd)/backend:/project cartos-backend

build-arm:
	docker build -f Dockerfile.rust.build.arm64 -t cartos-backend ./backend \
		&& make build-backend-arm-prod-exe
