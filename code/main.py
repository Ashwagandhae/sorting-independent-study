import time, random
from sorts import *


def gen_unsorted_arr(length: int, rang: int) -> [int]:
    return [random.randint(0, rang) for _ in range(length)]


TEST_ARRS = {
    "small": [gen_unsorted_arr(10, 100)],
    "medium": [gen_unsorted_arr(100, 1000)],
    "large": [gen_unsorted_arr(1000, 10000)],
}


def test_sort(
    fn: callable([int]),
    arr: [int],
) -> (bool, float):
    start_time = time.time()
    fn(arr)
    end_time = time.time()
    time_taken = end_time - start_time
    for i in range(len(arr) - 1):
        if arr[i] > arr[i + 1]:
            return False, time_taken
    return True, time_taken


def print_test_sort(fn: callable([int]), arr: [int]):
    success, time_taken = test_sort(fn, arr)
    ms = time_taken * 1000
    if success:
        print(f"    success {ms} ms")
    else:
        print(f"    failed! {ms} ms")


def print_test_sort_on_arrs(fn: callable([int]), arr: {str: [int]}):
    print(f"testing {fn.__name__}...")
    for name, arrs in arr.items():
        print(f"  {name}:")
        for arr in arrs:
            print_test_sort(fn, arr)


def test_sorts(fns: [callable([int])], arr: {str: [int]}):
    for fn in fns:
        print_test_sort_on_arrs(fn, arr)


if __name__ == "__main__":
    test_sorts(
        [
            insertion_sort,
        ],
        TEST_ARRS,
    )
