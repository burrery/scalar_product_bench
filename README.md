# scalar_product_bench
Benchmark of several implementation of the scalar product in rust.
I was surprised that rayon par_iter was more efficient only for vectors 
that contains more than 1 million elements
