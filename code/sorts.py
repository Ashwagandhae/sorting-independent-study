from util import swap


def insertion_sort(arr: [int]):
    for j in range(1, len(arr)):
        key = arr[j]
        # insert arr[j] into the sorted sequence arr[1..j-1]
        i = j - 1
        while i >= 0 and key < arr[i]:
            swap(arr, i, i + 1)
            i -= 1


def selection_sort(arr: [int]):
    for i in range(len(arr)):
        # get smallest element in arr[i..len(arr)]
        min_index = i
        for j in range(i + 1, len(arr)):
            if arr[min_index] > arr[j]:
                min_index = j
        # swap arr[i] and arr[min_index]
        swap(arr, i, min_index)


def merge_sort(arr: [int]):
    def merge(arr: [int], p: int, q: int, r: int):
        # arr[p..q] and arr[q+1..r] are sorted
        # merge them into arr[p..r]
        n1 = q - p + 1
        n2 = r - q
        left = [0] * n1
        right = [0] * n2
        for i in range(n1):
            left[i] = arr[p + i]
        for j in range(n2):
            right[j] = arr[q + j + 1]
        i = 0
        j = 0
        k = p
        # merge left and right into arr
        while i < n1 and j < n2:
            if left[i] <= right[j]:
                arr[k] = left[i]
                i += 1
            else:
                arr[k] = right[j]
                j += 1
            k += 1
        # copy remaining elements of left and right into arr
        while i < n1:
            arr[k] = left[i]
            i += 1
            k += 1
        while j < n2:
            arr[k] = right[j]
            j += 1
            k += 1

    def rec(arr: [int], p: int, r: int):
        if p < r:
            q = (p + r) // 2
            rec(arr, p, q)
            rec(arr, q + 1, r)
            merge(arr, p, q, r)

    rec(arr, 0, len(arr) - 1)


SORTS = [
    insertion_sort,
    selection_sort,
    merge_sort,
]
