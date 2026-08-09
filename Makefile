tc:
	RUST_MIN_STACK=33554432 cargo test --release test_examples_write_c -- --nocapture

tc-fuzz:
	RUST_MIN_STACK=33554432 python3 tests/fuzz.py --print -n80 --seed 22

tc-all:
	bash ./scripts/grade-write_c.sh
