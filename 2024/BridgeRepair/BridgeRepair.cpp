#include <fstream>
#include <iostream>
#include <vector>

enum OPERATOR {
  ADDITION,
  MULTIPLICATION,
  CONCATENATION
};

bool isValid(std::vector<long> vals, long target, int idx, long acc, OPERATOR op) {
  if (idx >= vals.size()) {
    return false;
  }

  switch (op) {
    case OPERATOR::ADDITION:
      acc = vals[idx] + acc;
      break;
    case OPERATOR::MULTIPLICATION:
      acc = vals[idx] * acc;
      break;
    case OPERATOR::CONCATENATION:
      std::string strAcc = std::to_string(acc);
      std::string strRhs = std::to_string(vals[idx]);
      acc = std::stol(strAcc.append(strRhs));
      break;
  }

  if (acc == target && idx == vals.size() - 1) {
    return true;
  }

  bool mulValid = isValid(vals, target, idx + 1, acc, OPERATOR::MULTIPLICATION);
  bool addValid = isValid(vals, target, idx + 1, acc, OPERATOR::ADDITION);
  bool concatValid = isValid(vals, target, idx + 1, acc, OPERATOR::CONCATENATION);

  return mulValid || addValid || concatValid;
}

int main(int argc, char* argv[]) {
  bool debug = false;
  bool sample = false;

  if (argc > 1) {
    for (int i = 1; i < argc; ++i) {
      std::string arg(argv[i]);
      for (int i = 0; i < arg.length(); ++i) {
        arg[i] = tolower(arg[i]);
      }

      if (arg == "sample") {
        sample = true;
      }

      if (arg == "debug") {
        debug = true;
      }
    }
  }

  std::string fname = sample ? "sampleinput.txt" : "input.txt";
  std::ifstream input(fname);
  if (!input.is_open()) {
    std::cerr << "ERROR Could not open file: " << fname << std::endl;
    exit(EXIT_FAILURE);
  }

  std::string line;
  long sum = 0;
  while (getline(input, line)) {
    long target;
    std::vector<long> nums;

    int delimIdx = line.find(':');
    target = std::stol(line.substr(0, delimIdx));
    line = line.substr(delimIdx + 1);

    std::string numStr;
    for (int i = 0; i < line.length(); ++i) {
      if (line[i] != ' ') {
        numStr.push_back(line[i]);
      } else if (!numStr.empty()) {
        nums.push_back(std::stol(numStr));
        numStr.clear(); 
      }
    }
    nums.push_back(std::stoi(numStr));

    if (debug) {
      std::cout << "Target: " << target << " ";
      std::cout << "Nums: ";
      for (long num : nums) {
        std::cout << num << " ";
      }
      std::cout << std::endl;
    }

    if (isValid(nums, target, 0, 0, OPERATOR::ADDITION)) {
      sum += target;
    }
  }

  std::cout << "Sum: " << sum << std::endl;

  exit(EXIT_SUCCESS);
}