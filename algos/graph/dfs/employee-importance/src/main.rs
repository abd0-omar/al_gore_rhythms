use std::collections::HashMap;

// there is no rust submission for this on leetcode unfortunately
// maybe problem with pointers..idk
// I figured out why, just problems with borrow checker due to passing Employee around
fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Employee {
    id: i32,
    importance: i32,
    subordinates: Vec<i32>,
}

struct Solution {
    id_to_emp: HashMap<i32, Employee>,
}

#[allow(unused)]
impl Solution {
    // add code here
    fn get_importance(&mut self, employees: Vec<Employee>, id: i32) -> i32 {
        let mut id_to_emp = HashMap::new();

        for emp in employees {
            id_to_emp.insert(emp.id, emp);
        }

        let solution = Solution { id_to_emp };

        solution.dfs(id)
    }

    fn dfs(&self, id: i32) -> i32 {
        let e = self.id_to_emp.get(&id).unwrap();

        let mut sum_importance = e.importance;

        for sub in &e.subordinates {
            sum_importance += self.dfs(*sub);
        }

        sum_importance
    }
}
