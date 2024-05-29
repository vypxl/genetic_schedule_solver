default: make-chart

generate-scratch:
  # cargo run -- generate <N_STUDENTS> <N_COURSES> <N_TIMESLOTS> <N_ROOMS> <N_COURSES_PER_STUDENT> <N_COURSES_PER_PROF>
  cargo run -- generate -o scratch.json 128 32 16 8 8 4

compute-scratch:
  # cargo run -- compute <POPULATION_SIZE> <N_GENERATIONS> <SELECTION_FACTOR> <MUTATION_CHANCE>
  cargo run --release -- compute -f scratch.json 1024 512 0.03 0.025

gen-small:
 cargo run --release -- generate 100 16 10 3 3 2 -o data/small.json

gen-medium:
 cargo run --release -- generate 256 50 20 5 8 5 -o data/medium.json

gen-realistic:
  cargo run --release -- generate 5000 100 40 10 20 10 -o data/realistic.json

gen-all: gen-small gen-medium gen-realistic

run-small population_size generations split mut_chance:
  cargo run --release -- compute -cf data/small.json {{population_size}} {{generations}} {{split}} {{mut_chance}}

run-medium population_size generations split mut_chance:
  cargo run --release -- compute -cf data/medium.json {{population_size}} {{generations}} {{split}} {{mut_chance}}

run-realistic population_size generations split mut_chance:
  cargo run --release -- compute -cf data/realistic.json {{population_size}} {{generations}} {{split}} {{mut_chance}}

run-all population_size generations split mut_chance:
  just run-small {{population_size}} {{generations}} {{split}} {{mut_chance}} | tee small.csv
  just run-medium {{population_size}} {{generations}} {{split}} {{mut_chance}} | tee medium.csv
  just run-realistic {{population_size}} {{generations}} {{split}} {{mut_chance}} | tee realistic.csv
  paste -d ',' small.csv medium.csv realistic.csv | tee all.csv

  rm small.csv medium.csv realistic.csv

make-chart:
  ./make_chart.py
