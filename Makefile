NAME=quizz

all: build
	./$(NAME) ./questions/quiz.txt

build:
	cargo build
	mv ./target/debug/quizz .

release:
	cargo build --release

run:
	cargo run -q

check:
	cargo check

clean:
	cargo clean

clippy:
	cargo clippy -- -D warnings

install: release
	install -Dm755 target/release/$(NAME) /usr/local/bin/$(NAME)

uninstall:
	rm -f /usr/local/bin/$(NAME)
