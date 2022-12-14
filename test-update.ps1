Push-Location ./test/a
& cargo run --bin eto -- patch -p ../../*.etopack --wait-for 17744 --on-complete notepad
Pop-Location
