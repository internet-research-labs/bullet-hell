.PHONY: all bundle run

BULLET_HELL=target/release/bullet-hell

BUILD_JS=www/static/build.js

all:

bundle: $(BUILD_JS) $(BULLET_HELL)

run: $(BUILD_JS)
	RUSTFLAGS="-A dead_code" cargo run -- -p 8080 --path www/static -w 30 -h 30

$(BULLET_HELL): src/*
	RUSTFLAGS="-A dead_code" cargo build --bin=bullet-hell

$(BUILD_JS): src-js/*
	npm run build

clean:
	rm -fr target $(BUILD_JS) package-lock.json Cargo.lock
