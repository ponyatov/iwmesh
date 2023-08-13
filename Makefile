.PHONY: server
server:
	cargo watch -w config -w lib -w $@ -x "run -p $@"
