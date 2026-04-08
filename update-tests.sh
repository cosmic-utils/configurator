
TRYBUILD=overwrite SNAPSHOTS=overwrite cargo test -p rust_schema2 --all-features --no-fail-fast --tests || :
