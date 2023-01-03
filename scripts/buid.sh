cargo build --release
cp ./target/release/kth ./scripts/kth
cd ./scripts
tar -czf kth.tar.gz kth
rm kth
cd ..
