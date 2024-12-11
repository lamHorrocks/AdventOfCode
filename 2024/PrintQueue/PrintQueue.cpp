#include <iostream>
#include <fstream>
#include <string>
#include <map>
#include <vector>
#include <set>
#include <functional>

int main() {
  std::string line;
  std::ifstream input("input.txt");

  if (!input.is_open()) {
    std::cerr << "Unable to open file" << std::endl;
    exit(EXIT_FAILURE);
  }

  // read rules into map
  std::map<int, std::vector<int>> ruleMap;
  while(getline(input, line) && !line.empty()) {
    size_t delimIdx = line.find("|");
    int num1 = std::stoi(line.substr(0, delimIdx));
    int num2 = std::stoi(line.substr(delimIdx + 1));

    auto iter = ruleMap.find(num1);
    if (iter != ruleMap.end()) {
      iter->second.push_back(num2);
    } else {
      ruleMap[num1] = std::vector<int> { num2 };
    }
  }

  // print map
  // printMap(ruleMap);

  // read updates
  std::vector<std::vector<int>> updates;
  while (getline(input, line)) {
    std::vector<int> update;

    while (!line.empty()) {
      size_t delimIdx = line.find(",");

      if (delimIdx != std::string::npos) {
        update.push_back(std::stoi(line.substr(0, delimIdx)));
        line = line.substr(delimIdx + 1);
      } else {
        update.push_back(std::stoi(line));
        line.clear();
      }
    }

    updates.push_back(update);
  }

  // print updates
  // printUpdates(updates);

  input.close();

  // validate and get middle values of valid updates
  int part1Sum = 0;
  int part2Sum = 0;
  for (auto update : updates) {
    std::set<int> ruleSet;
    bool valid = true;

    for (auto num : update) {
      auto iter = ruleMap.find(num);

      if (iter != ruleMap.end()) {
        std::vector<int> ruleEntries = iter->second;

        for (int num : ruleEntries) {
          if (ruleSet.find(num) != ruleSet.end()) {
            valid = false;
            break;
          }
        }
      }

      if (!valid) {
        break;
      }

      ruleSet.insert(num);
    }

    if (valid) {
      part1Sum += update[update.size() / 2];
    } else {
      auto compare = [ruleMap](int a, int b) {
        auto aIter = ruleMap.find(a);

        if (aIter != ruleMap.end()) {
          std::vector<int> aRules = aIter->second;

          if (std::find(aRules.begin(), aRules.end(), b) != aRules.end())  {
            return true;
          }
        }

        return false;
      };

      std::sort(update.begin(), update.end(), compare);
      part2Sum += update[update.size() / 2];
    }
  }

  std::cout << "Part 1 Sum: " << part1Sum << std::endl;
  std::cout << "Part 2 Sum: " << part2Sum << std::endl;


  exit(EXIT_SUCCESS);
}