This is a fork of the Sophia benchmarking to compare "normal" Sophia (FastGraph and LightGraph) with the HDT Sophia adapter.
I couldn't get everything to work so I leave out the other libraries.
The old values are still in the notebook, only the query part is updated.
The old and new values are not comparable because I test on an Intel i9-12900k.
The person data files need to be converted to HDT for the HDT test to work.
Original readme below.
Known bug: Logging text from HDT needs to be manually removed from the HDT CSV file.




# Benchmarking for Sophia

This is an environment for benchmarking the [sophia] library,
and comparing it with other RDF libraries.

[sophia]: https://github.com/pchampin/sophia_rs

## See the results

The results are available in [`benchmark_results.ipynb`](./benchmark_results.ipynb).
They should display correctly on github.
Otherwise, you need [Jupyter](http://jupyter.org/) to visualize them.

## Reproduce the results

The tests have been designed for my machine, running Ubuntu 18.10.
To load and build all the necessarily files,
type `make` in the root directory of the project
(see [`benchmark_results.ipynb`](./benchmark_results.ipynb) for dependencies).
To re-generate the CSV files,
use the `run_benchmark` command with the appropriate arguments.

### Further Requirements

#### n3js

    export NODE_OPTIONS=--max_old_space_size=16000

#### librdf

One of the following (depending on your distribution):

* pacman -S redland
* apt install librdf-dev

## Adding libraries to the benchmark

If you want to add another library to the benchmark,
have a look at the [`BENCHMARK_INTERFACE.txt`](./BENCHMARK_INTERFACE.txt) file.
