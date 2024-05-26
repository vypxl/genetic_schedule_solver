run: generate-scratch compute-scratch
  echo done

generate-scratch:
  # cargo run -- generate <N_STUDENTS> <N_COURSES> <N_TIMESLOTS> <N_ROOMS> <N_COURSES_PER_STUDENT> <N_COURSES_PER_PROF>
  cargo run -- generate -o scratch.json 128 32 16 8 8 4

compute-scratch:
  # cargo run -- compute <POPULATION_SIZE> <N_GENERATIONS> <SELECTION_FACTOR> <MUTATION_CHANCE>
  cargo run -- compute -f scratch.json 1024 2 0.02 0.01
