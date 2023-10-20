#include <algorithm>
#include <cassert>
#include <iostream>
#include <vector>

// test
// assert(([2, 5, 7, 9, 30], 9)), 3);
// assert(([2, 5, 7, 9, 30], 9)), 3); not sorted

template <typename T> void quicksort(std::vector<T> &arr, int left, int right) {
  if (left < right) {
    int pivot = arr[left];
    int low = left + 1;
    int high = right;

    for (int i = low; i <= high; i++) {
      while (low <= high && arr[high] >= pivot) {
        high--;
      }
      while (low <= high && arr[low] <= pivot) {
        low++;
      }
      if (low <= high) {
        std::swap(arr[low], arr[high]);
      } else {
        break;
      }
    }

    std::swap(arr[left], arr[high]);

    quicksort(arr, left, high - 1);
    quicksort(arr, low, right);
  }
}

int main() {
  // Test Case 1: Random unsorted array
  std::vector<int> arr1 = {3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5};
  int size1 = arr1.size();
  quicksort(arr1, 0, size1 - 1);
  assert(std::is_sorted(arr1.begin(), arr1.end()));

  // Test Case 2: Already sorted array (should not change)
  std::vector<int> arr2 = {1, 2, 3, 4, 5, 6, 7, 8, 9};
  int size2 = arr2.size();
  quicksort(arr2, 0, size2 - 1);
  assert(std::is_sorted(arr2.begin(), arr2.end()));

  // Test Case 3: Reverse sorted array (should be sorted in ascending order)
  std::vector<int> arr3 = {9, 8, 7, 6, 5, 4, 3, 2, 1};
  int size3 = arr3.size();
  quicksort(arr3, 0, size3 - 1);
  assert(std::is_sorted(arr3.begin(), arr3.end()));

  // Test Case 4: Array with duplicate elements
  std::vector<int> arr4 = {3, 1, 4, 1, 5, 3, 2, 4, 5, 6};
  int size4 = arr4.size();
  quicksort(arr4, 0, size4 - 1);
  assert(std::is_sorted(arr4.begin(), arr4.end()));

  std::cout << "All test cases passed." << std::endl;
  return 0;
}
