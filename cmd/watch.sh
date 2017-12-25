killall php
php -S localhost:8000 -t bin/ &
open http://localhost:8000
cargo watch -cq -w src/index.rs -s cmd/compile.sh
