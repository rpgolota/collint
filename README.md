# collint : (coll)ective (int)elligence
#### Based on "Agent-based models of collective intelligence" by S. M. Reia, A. C. Amado, and J. F. Fontanari.
#### Term project for MATH 485
---

## Download from Releases
Download from releases to get a collint executable for 64-bit windows. No other targets are built against.

## How to install from source
There are two ways to use collint.
One way is to build and run an executable that works with config files shown in [collint/default](https://github.com/rpgolota/collint/tree/master/default).
Another way is to install the python wrapper around collint and use it in python scripts. The python wrapper is located in [collint/python](https://github.com/rpgolota/collint/tree/master/python).

### collint executable
Running this executable will write results to a csv file specified in the configuration file.
The csv columns are explained in the configuration files in [collint/default](https://github.com/rpgolota/collint/tree/master/default).
1) Install [rust](https://www.rust-lang.org/)
2) Clone this repository with `git clone https://github.com/rpgolota/collint.git`
3) Enter the root of the repository with `cd collint`
4) Build this repository with `cargo build --release`
5) The executable will be located at `target/release/collint(.exe)`
6) Run the executable in target or alternatively run `cargo run --release -- <args>`

#### collint arguments
`collint <RUN_TYPE> <CONFIG_PATH>`
- `RUN_TYPE` can be either:
  - `b` or `blackboard` to run the blackboard method
  - `i` or `imitative` to run the imitative method
- `CONFIG_PATH` is optional
  - Searches for either `blackboard_default.toml` oor `imitative_default.toml` if nothing is provided
  - Provided path is relative to the directory you run `collint` from.

### collint Python library
1) Install [rust](https://www.rust-lang.org/)
2) Install python-3.8 or higher version
3) Run `python -m pip install "git+https://github.com/rpgolota/collint.git#subdirectory=python"`
Alternatively...
3) Clone this repository with `git clone https://github.com/rpgolota/collint.git`
4) Enter the directory of the python code with `cd collint/python`
5) Install the contents of [collint/python](https://github.com/rpgolota/collint/tree/master/python) with `python -m pip install .`

#### How to use the collint Python library
A working example is shown in [collint/python/example.py](https://github.com/rpgolota/collint/blob/master/python/example.py)

The following are the exports of collint.
- Experiment - A class that can run both blackboard and imitation experiments

```python
from collint import Experiment

class Experiment:
    def __init__(): ...
    # disabled type checking for this experiment
    # if originally enabled before experiment, it will be turned on once experiment is finished
    def disable_type_checking(self): ...
    # enabled type checking (enabled by default, included here for completeness)
    def max_c(self, max_c): ...
    # enabled parallel running
    def parallel(self, show_progress=True): ...
    # set up blackboard method
    def blackboard(self, ms, bs, n, compute_phi=False): ...
    # set up imitative method
    def imitative(self, ms, ps, n): ...
    # run the experiment, and return results
    # if the experiment is non-parallel, what is actually returned is a generator
    # calling list() on this generator will give a list of all the results
    # otherwise you can just treat it as a normal python generator
    def run(self): ...
```
```python
### Running an imitative experiment
from collint import Experiment

M = range(7, 10 + 1) # using range, so disabled type checking
P = [0.5, 0.6]
N = 1

results = (
    Experiment()
    .disable_type_checking()
    .max_c(1.0) # chaging max_c from default (10.0), so need to call this method
    .parallel(show_progress=True) # parallel is false by default, but show_progress is true by default
    .imitative(M, P, N)
    .run()
) # Results are a list of either a dictionary or None if we exceed computational cost of 1.0
```

```python
### Running a blackboard experiment
from collint import Experiment

M = range(7, 10 + 1) # using range, so disabled type checking
B = range(6, 8 + 1)
N = 10

results = (
    Experiment()
    .disable_type_checking()
    .parallel() # show progress is true by default
    .blackboard(M, B, N, compute_phi = True) # compute_phi False by default
    .run()
) # Results are a list of either a dictionary or None if we exceed computational cost of 10.0
```

```python
### Running a non-parallel blackboard experiment, works the same for imitative
from collint import Experiment

M = range(7, 10 + 1) # using range, so disabled type checking
B = range(6, 8 + 1)
N = 10

results = (
    Experiment()
    .disable_type_checking()
    .blackboard(M, B, N, compute_phi = True) # compute_phi False by default
    .run()
) # Results is a generator

# Can use next() since results is a generator
first_result = next(results)

# Can use in a loop
for result in results:
    print(result) # result is either dictionary or None

M = range(7, 10 + 1)
B = range(6, 8 + 1)
N = 10

# Running experiment again...
results = (
    Experiment()
    .disable_type_checking()
    .blackboard(M, B, N, compute_phi = True) # compute_phi False by default
    .run()
)

# To run all the experiments sequentially without using a loop or next...
# ... use list()!

results = list(results) # experiment actually runs here

M = range(7, 10 + 1)
B = range(6, 8 + 1)
N = 10

# Running experiment again...
results = (
    Experiment()
    .disable_type_checking()
    .max_c(0.2) # set to a low value, we will probably have some Nones
    .blackboard(M, B, N, compute_phi = True) # compute_phi False by default
    .run()
)

# Alternatively, to get rid of all the None values...
# ... use list comprehension!!!
results = [r for r in results if r is not None]
```

- blackboard - A module that contains both parallel and non-parallel blackboard functions
Has two exports for running the blackboard method.
The parallel version runs to completion and only then returns the results.
The parallel version can show a progress bar with show_progress == True
```python
from collint.blackboard import blackboard, blackboard_parallel

# returns a dictionary with keys ("m", "b", "t_star", "c"). "phi" is only present if compute_phi == True
# If c exceeds max_c, result will be None
def blackboard(m, b, /, *, max_c = 10.0, compute_phi = False): ...
# returns a list of the dictionaries or None values. Keys are the same as blackboard()
def blackboard_parallel(ms, bs, n, /, *, max_c = 10.0, compute_phi = False, show_progress = True): ...

### Example of how to run
result = blackboard(10, 7, max_c = 9.0, compute_phi = True)
results = blackboard_parallel([8, 9, 10], [5, 6, 7], 10, max_c = 9.0, compute_phi = True, show_progress = False)
```

- imitative - A module that contains both parallel and non-parallel imitative functions
Has two exports for running the imitative method.
The parallel version runs to completion and only then returns the results.
The parallel version can show a progress bar with show_progress == True
```python
from collint.imitative import imitative, imitative_parallel

# returns a dictionary with keys ("m", "p", "t_star", "c")
# If c exceeds max_c, result will be None
def imitative(m, p, /, *, max_c = 10.0): ...
# returns a list of the dictionaries or None values. Keys are the same as imitative()
def imitative_parallel(ms, ps, n, /, *, max_c = 10.0, show_progress = True): ...

### Example of how to run
result = imitative(10, 0.7, max_c = 9.0)
results = imitative_parallel([8, 9, 10], [0.5, 0.6, 0.7], 10, max_c = 9.0, show_progress = False)
```

- config - A module that is responsible for configuring some aspects of collint
Type checking happens before the underlying rust code is called.
It is enabled by default, and disabling it might allow for some more pythonic uses of the collint library.
An example of when it might be good to disable it is when using range() as an input to the parallel functions.
Disabling this and passing in a wrong type of value to a function might cause the error to be less readable since it will be called from the compiled library.
```python
from collint.config import (
    is_python_type_checking_enabled,
    disable_python_type_checking,
    enable_python_type_checking
)

def disable_python_type_checking(): ...
def enable_python_type_checking(): ...
def is_python_type_checking_enabled(): ...

# check if type checking is enabled
enabled = is_python_type_checking_enabled()

# if enabled disable, if disabled enable
if enabled:
    disable_python_type_checking()
else:
    enable_python_type_checking()

```