#include <algorithm>
#include <iostream>
#include <queue>
using namespace std;

int main() {}
class KthNumberProcessor {
private:
  int k;
  priority_queue<int> q;

public:
  KthNumberProcessor(int k) : k(k) {}

  int next(int new_num) {
    if (q.size() < k) {
      q.push(new_num);
    }

    if (new_num < q.top()) {
      q.pop();
      q.push(new_num);
    }
    return q.top();
  }

  // int next(int new_num) {
  //
  // 	if(q.sz() < k)
  // 		q.push(new_num);
  // 	else if (new_num < q.top()) {
  // 		q.pop();
  // 		q.push(new_num);
  // 	}
  // 	return q.top();
  // }
};
// int next(int new_num) {
//
// 	if(q.sz() < k)
// 		q.push(new_num);
// 	else if (new_num < q.top()) {
// 		q.pop();
// 		q.push(new_num);
// 	}
// 	return q.top();
// }
