CARGO = cargo


.PHONY: test fmt build


test:
	$(CARGO) test

fmt:
	$(CARGO) fmt

build:
	$(CARGO) build --release
