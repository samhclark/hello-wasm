# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

# Runs clippy on the sources 
check:
	cargo clippy --locked -- -D warnings

# Runs unit tests
test:
	cargo test --locked

# Builds wasm module and links with NPM
build:
    wasm-pack build --target bundler
    cd {{justfile_directory()}}/pkg && npm link 
    cd {{justfile_directory()}}/site && npm link hello-wasm && npm install
    cd {{invocation_directory()}}

# Runs the dev server
run: build
    cd {{justfile_directory()}}/site && npm run serve
    
