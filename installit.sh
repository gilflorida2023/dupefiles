#/home/minty/.cargo/bin:/home/minty/projects/bin:...

export tomlloc=$(pwd)
cargo build -r
cargo test -r
cd target/release
cargo install --path $tomlloc
cd
dupefiles  
