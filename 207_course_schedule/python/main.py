from typing import List
import cProfile
from collections import deque


class Solution:
    def canFinish(self, numCourses: int, prerequisites: List[List[int]]) -> bool:
        def vertex_descendants_iterator(vertex):
            for descendant in descendants[vertex]:
                yield descendant

        def dfs_visit(vertex: int) -> bool:
            white.remove(vertex)
            gray.add(vertex)

            # for edge in prerequisites:
            #     if edge[0] == vertex:
            #         descendant = edge[1]
            # for descendant in vertex_descendants_iterator(vertex):
            for descendant in descendants[vertex]:
                if descendant in white:
                    acyclic = dfs_visit(descendant)
                    if acyclic == False:
                        return False

                elif descendant in gray:
                    return False

            gray.remove(vertex)
            black.add(vertex)
            return True

        white = set(range(numCourses))
        gray = set()
        black = set()

        descendants = {
            i: [edge[1] for edge in prerequisites if edge[0] == i]
            for i in range(numCourses)
        }

        for course_num in range(numCourses):
            if course_num in white:
                flag = dfs_visit(course_num)
                if flag == False:
                    return False

        return True


random_arcs = [
    (2, [[1, 0]]),
    (2, [[1, 0], [0, 1]]),
    (3, [[1, 0], [2, 1]]),
    (3, [[1, 0], [2, 1], [0, 2]]),
    (4, [[1, 0], [2, 1], [3, 2], [0, 3]]),
    (4, [[1, 0], [2, 1], [3, 2], [0, 3], [1, 3]]),
    (4, [[1, 0], [2, 1], [3, 2], [0, 3], [1, 3], [2, 3]]),
    (4, [[1, 0], [2, 1], [3, 2], [0, 3], [1, 3], [2, 3], [3, 0]]),
    (4, [[1, 0], [2, 1], [3, 2], [0, 3], [1, 3], [2, 3], [3, 0], [0, 1]]),
    (4, [[1, 0], [2, 1], [3, 2], [0, 3], [1, 3], [2, 3], [3, 0], [0, 1], [1, 2]]),
    (5, [[1, 0], [2, 1], [3, 2], [4, 3], [0, 4]]),
    (
        10,
        [
            [1, 0],
            [2, 1],
            [3, 2],
            [4, 3],
            [5, 4],
            [6, 5],
            [7, 6],
            [8, 7],
            [9, 8],
            [0, 9],
        ],
    ),
    (100, [[1, 0]]),
    (100, [[1, 0], [2, 1]]),
    (
        15,
        [
            [1, 0],
            [2, 1],
            [3, 2],
            [4, 3],
            [5, 4],
            [6, 5],
            [7, 6],
            [8, 7],
            [9, 8],
            [10, 9],
            [11, 10],
            [12, 11],
            [13, 12],
            [14, 13],
            [0, 14],
        ],
    ),
]
s = Solution()

with cProfile.Profile() as pr:
    for size, g in random_arcs:
        s.canFinish(size, g)
        # dump profile to file
    pr.dump_stats("profile.prof")
