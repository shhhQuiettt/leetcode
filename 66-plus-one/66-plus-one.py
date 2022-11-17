from typing import List


class Solution:
    def plusOne(self, digits: List[int]) -> List[int]:
        i = len(digits) - 1
        while digits[i] == 9:
            digits[i] = 0
            if i == 0:
                digits.insert(0, 1)
                return digits
            i -= 1

        digits[i] += 1
        return digits


# import collections
# class Solution:
#     def plusOne(self, digits: List[int]) -> List[int]:
#         d = collections.deque(digits)
#         i = len(digits) - 1
#         while d[i] == 9:
#             d[i] = 0
#             if i == 0:
#                 d.appendleft(1)
#                 return d
#             i -= 1

#         d[i] += 1
#         return d


def test_plusOne():
    cases = [[1, 2, 3], [4, 3, 2, 1], [9]]
    results = [[1, 2, 4], [4, 3, 2, 2], [1, 0]]
    s = Solution()
    for case, result in zip(cases, results):
        assert s.plusOne(case) == result
