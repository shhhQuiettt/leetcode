struct Solution;

use std::collections::VecDeque;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let side_length = grid.len() as i32;

        if grid[0][0] != 0 || grid[side_length as usize - 1][side_length as usize - 1] != 0 {
            return -1;
        }

        let mut vertex_que: VecDeque<((i32, i32), i32)> = VecDeque::new();
        let mut visited = vec![vec![false; side_length as usize]; side_length as usize];

        vertex_que.push_front(((0, 0), 1));
        visited[0][0] = true;

        let destination_vertex = (grid.len() as i32 - 1, grid.len() as i32 - 1);

        let directions: Vec<(i32, i32)> = vec![
            (1, 1),
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
            (1, -1),
            (-1, -1),
            (-1, 1),
        ];

        while let Some((vertex, path_length)) = vertex_que.pop_front() {
            if vertex == destination_vertex {
                return path_length;
            };


            for direction in directions.iter() {
                if vertex.0 == 0 && direction.0 == -1
                    || vertex.1 == 0 && direction.1 == -1
                    || vertex.0 == side_length - 1 && direction.0 == 1
                    || vertex.1 == side_length - 1 && direction.1 == 1
                {
                    continue;
                }

                let new_vertex = (vertex.0 + direction.0, vertex.1 + direction.1);

                if visited[new_vertex.0 as usize][new_vertex.1 as usize] {
                    continue;
                }

                if grid[new_vertex.0 as usize][new_vertex.1 as usize] == 0 {
                    visited[new_vertex.0 as usize][new_vertex.1 as usize] = true;
                    vertex_que.push_back((new_vertex, path_length + 1));
                }
            }
        }

        return -1;
    }
}
fn main() {
    assert_eq!(
        Solution::shortest_path_binary_matrix(vec![vec![0, 0], vec![0, 0]]),
        2
    );
    assert_eq!(
        Solution::shortest_path_binary_matrix(vec![vec![0, 1], vec![1, 0]]),
        2
    );
    assert_eq!(
        Solution::shortest_path_binary_matrix(vec![vec![0, 0, 0], vec![1, 1, 0], vec![1, 1, 1]]),
        -1
    );
    // assert_eq!(
    //     Solution::shortest_path_binary_matrix(vec![vec![1, 0, 0], vec![1, 1, 0], vec![1, 1, 0]]),
    //     -1
    // );
}
