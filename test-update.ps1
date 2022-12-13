Push-Location ./test/a
& cargo run --bin eto -- patch -p ../../*.etopack
Pop-Location
