default:
  @just --list

bench-all:
    cargo bench -q > benchmarks.txt

#where day like y2024d01 and part is 1 or 2
bench day part:
    cargo bench --bench {{day}} part{{part}}

#where day like y2024d01 and part is 1 or 2
flamegraph day part:
    cargo flamegraph --profile flamegraph --root --package {{day}} --bin part{{part}} --image-width 800 -o flamegraph.svg

#where day like y2024d01 and part is 1 or 2
dhat day part:
    cargo run --profile dhat --features dhat-heap --package {{day}} --bin part{{part}}

#day like 1
create day:
    #!/usr/bin/env bash
    formatted_day=$(printf "y%sd%02d" "$(date +%Y)" {{day}})
    mkdir -p "y$(date +%Y)"
    cd "y$(date +%Y)" && cargo generate --path ../template --name "${formatted_day}"
    just get-input {{day}}
    git add "${formatted_day}"

#day like 1
get-input day:
    #!/usr/bin/env bash
    session=$(grep -E '^SESSION=' .env | cut -d'=' -f2)
    formatted_day=$(printf "y%sd%02d" "$(date +%Y)" {{day}})
    curl -b "session=${session}" -o "y$(date +%Y)/${formatted_day}/src/input" "https://adventofcode.com/$(date +%Y)/day/{{day}}/input"