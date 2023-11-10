from typing import Union, List


# class Heap:
#     def __init__(self, value, left, right):
#         self.value: int = value
#         self.left_child: Union[Heap, None] = left
#         self.right_child: Union[Heap, None] = right

#     def max_heapify(self):
#         if self.left_child != None:
#             self.left_child.max_heapify()
#         if self.right_child != None:
#             self.right_child.max_heapify()
#         if self.left_child != None and self.left_child.value > self.value:
#             self.value, self.left_child.value = self.left_child.value, self.value
#         if self.right_child != None and self.right_child.value > self.value:
#             self.value, self.right_child.value = self.right_child.value, self.value

#     def draw(self) -> str:
#         return f"({self.left_child.draw() if self.left_child != None else 'null'} <- {self.value} -> {self.left_child.draw() if self.left_child != None else 'null'})"


# heap = Heap(
#     1,
#     Heap(2, Heap(4, None, None), Heap(5, None, None)),
#     Heap(3, Heap(6, None, None), Heap(7, None, None)),
# )

# print(heap.draw())


class Heap:
    def __init__(self, arr: List[int]):
        self.arr: List[int] = arr
        self.size: int = len(arr)
        self.build_max_heap()

    def build_max_heap(self):
        for i in range(self.size // 2, -1, -1):
            self.max_heapify(i)

    def max_heapify(self, i: int):
        left = 2 * i + 1
        right = 2 * i + 2
        largest = i
        if left < self.size and self.arr[left] > self.arr[largest]:
            largest = left
        if right < self.size and self.arr[right] > self.arr[largest]:
            largest = right
        if largest != i:
            self.arr[i], self.arr[largest] = self.arr[largest], self.arr[i]
            self.max_heapify(largest)

    def draw(self):
        sizes = [0, 2, 6, 14, 30, 62, 126, 254]
        size_class = 0
        for i in range(0, len(sizes)):
            if self.size >= sizes[i]:
                size_class = i + 2

        for i in range(0, self.size):
            spacing_count = (2 ** (size_class + 1)) - 1
            if i in [0, 1, 3, 7, 15, 31, 63, 127, 254]:
                size_class -= 1
                spacing_count = (2 ** (size_class)) - 1
                print()
            print(f"{' ' * int(spacing_count)}{self.arr[i]}", end="")


heap = Heap([1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5, 6, 7, 8, 9])


def heap_sort(arr: List[int]):
    heap = Heap(arr)
    for i in range(len(arr) - 1, -1, -1):
        arr[0], arr[i] = arr[i], arr[0]
        heap.size -= 1
        heap.max_heapify(0)
    return arr


print(heap_sort([1, 3, 2, 4, 5, 6, 7, 8, 9, 1, 2, 3]))
