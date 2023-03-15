#include <iostream>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

class Solution {
public:
  vector<int> twoSum(vector<int> &nums, int target) {

    unordered_map<int, int> looking_for;

    for (int i = 0; i < nums.size(); ++i) {
      if (looking_for.count(target - nums[i])) {
        return {looking_for[target - nums[i]], i};
      }
      looking_for[nums[i]] = i;
    }

    return {-1, -1};
  }
};

int main() {
  Solution s = Solution();
  vector<int> nums = {2, 7, 11, 15};
  vector<int> result = s.twoSum(nums, 9);

  cout << result[0] << " " << result[1] << std::endl;

  return 0;
}
