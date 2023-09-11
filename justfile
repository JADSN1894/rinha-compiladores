# List all commands availble
list:
    just --list

# Audit binaries for known bugs or security vulnerabilities in production, at scale, with zero bookkeeping.
audit:
    @cargo auditable build --release --target=x86_64-unknown-linux-musl
    @cargo audit bin ./target/x86_64-unknown-linux-musl/release/xpto

# This tool helps Rust users evaluate the quality and trustworthiness of their package dependencies.
crev:
    @cargo crev trust --level high https://github.com/dpc/crev-proofs
    @cargo crev repo fetch all
    @cargo crev verify --show-all