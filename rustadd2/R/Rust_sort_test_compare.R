X <- sample.int(1000000000, 100000000)

start_time <- proc.time()
sort(X)
end_time <- proc.time()
R_Time <- end_time - start_time


start_time <- proc.time()
rust_sort(X)
end_time <- proc.time()
Rust_Time <- end_time - start_time

Rust_Time/R_Time
