struct Solution;

use std::collections::VecDeque;

#[derive(Clone, Copy)]
enum VertexState {
    NotVisited,
    Discovering,
    Finished,
}

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;

        let mut course_order: VecDeque<i32> = VecDeque::with_capacity(num_courses);

        let mut graph: Vec<Vec<i32>> = vec![Vec::new(); num_courses];

        for edge in &prerequisites {
            graph[edge[0] as usize].push(edge[1]);
        }

        let mut vertices_state: Vec<VertexState> = vec![VertexState::NotVisited; num_courses];

        for course_id in 0..num_courses {
            let non_cycle =
                Self::dfs_sort_visit(course_id, &graph, &mut vertices_state, &mut course_order);
            if !non_cycle {
                println!("aaa");
                return vec![];
            }
        }
        return Vec::from(course_order);
    }

    fn dfs_sort_visit(
        course_id: usize,
        graph: &Vec<Vec<i32>>,
        vertices_state: &mut Vec<VertexState>,
        course_order: &mut VecDeque<i32>,
    ) -> bool {
        match vertices_state[course_id] {
            VertexState::Finished => return true,
            VertexState::NotVisited => {
                vertices_state[course_id] = VertexState::Discovering;

                if graph[course_id].iter().any(|descendant_course_id| {
                    !Self::dfs_sort_visit(
                        *descendant_course_id as usize,
                        graph,
                        vertices_state,
                        course_order,
                    )
                }) {
                    return false;
                }

                vertices_state[course_id] = VertexState::Finished;
                course_order.push_back(course_id as i32);
                return true;
            }

            VertexState::Discovering => return false,
        }
    }
}

fn main() {
    let g = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
    let o = Solution::find_order(4, g);
    println!("{:#?}", o);

    let g = vec![vec![1, 0], vec![0, 1]];
    let o = Solution::find_order(4, g);
    println!("{:#?}", o);
    


}
