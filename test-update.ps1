Push-Location ./test/a
& cargo run --bin eto -- patch -p ../../*.etopack --wait_for 15244 --on_complete taskmgr
Pop-Location
