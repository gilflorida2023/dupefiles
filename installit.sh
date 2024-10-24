#/home/minty/.cargo/bin:/home/minty/projects/bin:...

export tomlloc=$(pwd)
cargo build -r -v
cargo test -r -v
cd target/release
cargo install --path $tomlloc
cd
dupefiles  
