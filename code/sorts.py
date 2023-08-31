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
