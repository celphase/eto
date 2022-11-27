Push-Location ./test/a
& cargo run --bin eto -- auto-patch -p ../../*.etopack
Pop-Location
