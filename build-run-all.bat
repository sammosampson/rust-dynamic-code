@echo off

cd app
cargo build
cd ..
copy .\app\target\debug\app.dll .\runner\target\ 
cd runner
cargo run 
