.PHONY: all
all:

.PHONY: format
format:
	cargo fmt

WATCH = cargo watch -w config -w lib
.PHONY: server
server:
	$(WATCH) -w $@ -x "run -p $@"
