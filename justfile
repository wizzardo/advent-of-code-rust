default:
  @just --list

bench-all:
    cargo bench -q > benchmarks.txt

#where day like y2024d01 and part is 1 or 2
bench day part:
    cargo bench --bench {{day}} part{{part}}

#where day like y2024d01
bench-day day:
    cargo bench --bench {{day}}

#where day like y2024d01 and part is 1 or 2
flamegraph day part:
    cargo flamegraph --profile flamegraph --root --package {{day}} --bin part{{part}} --image-width 1800 -o flamegraph.svg

#where day like y2024d01 and part is 1 or 2
dhat day part:
    cargo run --profile dhat --features dhat-heap --package {{day}} --bin part{{part}}

##day like 1
#create day:
#    #!/usr/bin/env bash
#    formatted_day=$(printf "y%sd%02d" "$(date +%Y)" {{day}})
#    mkdir -p "y$(date +%Y)"
#    cd "y$(date +%Y)" && cargo generate --path ../template --name "${formatted_day}"
#    just get-input {{day}}
#    git add "${formatted_day}"
#
##day like 1
#get-input day:
#    #!/usr/bin/env bash
#    session=$(grep -E '^SESSION=' .env | cut -d'=' -f2)
#    formatted_day=$(printf "y%sd%02d" "$(date +%Y)" {{day}})
#    curl -b "session=${session}" -o "y$(date +%Y)/${formatted_day}/src/input" "https://adventofcode.com/$(date +%Y)/day/{{day}}/input"

#day like 2024 1
create year day:
    #!/usr/bin/env bash
    formatted_day=$(printf "y%sd%02d" {{year}} {{day}})
    mkdir -p "y{{year}}"
    cd "y{{year}}" && cargo generate --path ../template --name "${formatted_day}"
    just get-input {{year}} {{day}}
    git add "${formatted_day}"

#day like 2024 1
get-input year day:
    #!/usr/bin/env bash
    session=$(grep -E '^SESSION=' .env | cut -d'=' -f2)
    formatted_day=$(printf "y%sd%02d" {{year}} {{day}})
    curl -b "session=${session}" -o "y{{year}}/${formatted_day}/src/input" "https://adventofcode.com/{{year}}/day/{{day}}/input"

create-ebc day:
    #!/usr/bin/env bash
    formatted_day=$(printf "ebc-y%sd%02d" "$(date +%Y)" {{day}})
    mkdir -p "ebc-y$(date +%Y)"
    cd "ebc-y$(date +%Y)" && cargo generate --path ../ebc-template --name "${formatted_day}"
#    git add "${formatted_day}"