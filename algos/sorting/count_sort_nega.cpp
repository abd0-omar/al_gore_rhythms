#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;
vector<int> sortArray(vector<int> &nums) {
  vector<int> freq(50000 + 50000 + 1, 0);
  // vector<int> rez(nums.size());
  vector<int> rez;
  // freq.resize(50000 + 50000 + 1);
  // for (int i = 0; i < freq.size(); i++) {
  //   cout << freq[i] << "\n";
  // }

  for (int i = 0; i < nums.size(); i++) {
    freq[nums[i] + 50000]++;
  }

  for (int i = 0; i < freq.size(); i++) {
    while (freq[i] > 0) {
      // rez[i - 50000] = i;
      rez.push_back(i - 50000);
      freq[i]--;
    }
  }

  for (int i = 0; i < rez.size(); i++) {
    cout << rez[i] << "\n";
  }

  return rez;
}
int main() {
  /*
* Input: nums = [-5, 2, -3, 1, 1234, -2453]
● Output: [-2453, -5, -3, 1, 2, 1234]
● Online judge: LeetCode 912 - Sort an Arr
* */
  vector<int> nums = {-5, 2, -3, 1, 1234, -2453, -50000, 50000};
  vector<int> res = sortArray(nums);
}
