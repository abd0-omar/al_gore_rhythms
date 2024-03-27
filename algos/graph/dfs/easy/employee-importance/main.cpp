#include <iostream>
#include <unordered_set>
#include <vector>
using namespace std;

int main() {}

class Employee {
public:
  int id;
  int importance;
  vector<int> subordinates;
};

int getImportance(vector<Employee *> employees, int id) {}

void dfs(vector<Employee *> employees, unordered_set<Employee> visited,
         int id) {
  Employee *e = new Employee;

  visited.insert(e);
}
