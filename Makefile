test:
	@cargo test

coverage:
	@cargo tarpaulin --ignore-tests --out=Html --output-dir=./target/debug
	@open ./target/debug/tarpaulin-report.html
