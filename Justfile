set dotenv-load
set positional-arguments

pipe:
    cargo build --release --bin pipe_child
    cp $CARGO_TARGET_DIR/release/pipe_child ./
    cargo build --release --bin pipe_parent
    cp $CARGO_TARGET_DIR/release/pipe_parent ./

shm:
    cargo build --release --bin shm_child
    cp $CARGO_TARGET_DIR/release/shm_child ./
    cargo build --release --bin shm_parent
    cp $CARGO_TARGET_DIR/release/shm_parent ./

run_shm:
    ./shm_parent

run_pipe:
    ./pipe_parent