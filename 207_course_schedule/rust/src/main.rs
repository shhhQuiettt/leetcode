struct Solution;

#[derive(Clone)]
enum Node {
    ToVisit,
    Discovering,
    Finished,
}

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut graph: Vec<Vec<i32>> = vec![vec![]; num_courses as usize];

        for arc in prerequisites {
            graph[arc[0] as usize].push(arc[1]);
        }

        let mut courses: Vec<Node> = vec![Node::ToVisit; num_courses as usize];


        (0..num_courses).all(|course_id| Self::dfs_visit(course_id as usize, &graph, &mut courses))
    }

    fn dfs_visit(course_id: usize, graph: &Vec<Vec<i32>>, nodes: &mut Vec<Node>) -> bool {
        let node: &Node = &nodes[course_id];

        match node {
            Node::Discovering => {
                return false;
            }
            Node::Finished => return true,
            Node::ToVisit => {
                nodes[course_id] = Node::Discovering;
                if graph[course_id].iter().any(|descendant_course_id| !Self::dfs_visit(*descendant_course_id as usize, graph, nodes)) {
                    return false;
                }
                nodes[course_id] = Node::Finished;
                return true;
            }
        }
    }
}
fn main() {
    println!("{}", Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    println!("{}", Solution::can_finish(2, vec![vec![0, 1]]));
}
