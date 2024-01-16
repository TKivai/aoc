work day part:
    cargo watch -x "check -p {{day}}" -s "just test {{part}} -p {{day}}" -s "just lint {{day}}"
lint day:
    cargo clippy -p {{day}}
test part +FLAGS='-p day1':
    cargo nextest run {{FLAGS}} {{part}}