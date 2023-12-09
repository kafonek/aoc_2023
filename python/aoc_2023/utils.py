import time
from pathlib import Path
from typing import Callable


def run_and_time(run_fn: Callable[[Path], str], fp: Path):
    """
    Helper function to time how long it takes to run a function and print out some information
     - Input file path
     - Output answer
     - Time elapsed
    """
    print(f"Reading data from: {fp}")
    start = time.perf_counter()
    answer = run_fn(fp)
    elapsed = time.perf_counter() - start
    print(f"Answer: {answer}")
    if elapsed < 1e-6:  # less than 1 microsecond
        print(f"Time: {elapsed * 1e9:.2f} ns")
    elif elapsed < 1e-3:  # less than 1 millisecond
        print(f"Time: {elapsed * 1e6:.2f} μs")
    elif elapsed < 1:  # less than 1 second
        print(f"Time: {elapsed * 1e3:.2f} ms")
    else:
        print(f"Time: {elapsed:.2f} s")
