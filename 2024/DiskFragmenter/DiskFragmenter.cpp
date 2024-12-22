#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <algorithm>

long calcChecksum(std::vector<int> filesystem) {
  long checksum = 0;
  
  for (int i = 0; i < filesystem.size(); ++i) {
    if (filesystem[i] != -1) {
      checksum += filesystem[i] * i;
    }
  }

  return checksum;
}

int main (int argc, char* argv[]) {
  bool sample = false;  
  bool debug = false;

  for (int i = 1; i < argc; ++i) {
    std::string arg(argv[i]);

    for (int j = 0; j < arg.length(); ++j) {
      arg[j] = tolower(arg[j]);
    }

    if (arg == "sample") {
      sample = true;
    }

    if (arg == "debug") {
      debug = true;
    }
  }

  std::string fname = sample ? "sampleinput.txt" : "input.txt";
  std::ifstream input(fname);

  if (!input.is_open()) {
    std::cerr << "ERROR Could not open file: " + fname << std::endl;
    exit(EXIT_FAILURE);
  }

  std::string line;
  getline(input, line);
  std::vector<int> blocks;
  for (int i = 0; i < line.length(); ++i) {
    int num = std::stoi(line.substr(i, 1));
    
    for (int j = 0; j < num; ++j) {
      if (i % 2 == 0) {
        blocks.push_back(i / 2);
      } else {
        blocks.push_back(-1);
      }
    }
  }

  std::vector<int> blocksP1(blocks.begin(), blocks.end());
  int l = 0;
  int r = blocksP1.size() - 1;
  while (l < r) {
    while(l < r && blocksP1[l] != -1) {
      ++l;
    }

    if (blocksP1[r] != -1 && l != r) {
      blocksP1[l] = blocksP1[r];
      blocksP1[r] = -1;
    }

    --r;
  }

  if (debug) {
    std::cout << "Part 1 Filesystem Result " << std::endl;
    for (int n : blocksP1) {
      if (n == -1) {
        std::cout << ".";
      } else {
        std::cout << n;
      }
    }
    std::cout << std::endl;
  }

  std::vector<int> blocksP2(blocks.begin(), blocks.end());
  l = 0;
  r = blocksP2.size() - 1;
  while (l < r) {
    while (l < r && blocksP2[l] != -1) {
      ++l;
    }

    while (l < r && blocksP2[r] == -1) {
      --r;
    }

    std::vector<int> fileIds;
    int currNum = blocksP2[r];
    while (l < r && blocksP2[r] == currNum) {
      fileIds.push_back(currNum);
      --r;
    }

    int ll = l;
    int gapLen = 0;
    while (ll <= r) {
      if (blocksP2[ll] == -1) {
        ++gapLen;
      } else {
        gapLen = 0;
      }

      if (gapLen == fileIds.size()) {
        for (int i = ll - gapLen + 1; i <= ll; ++i) {
          blocksP2[i] = currNum;
        }

        for (int i = r + gapLen; i > r; --i) {
          blocksP2[i] = -1;
        }

        break;
      }

      ++ll;
    }
  }

  if (debug) {
    std::cout << "Part 2 Filesystem Result " << std::endl;
    for (int n : blocksP2) {
      if (n == -1) {
        std::cout << ".";
      } else {
        std::cout << n;
      }
    }
    std::cout << std::endl;
  }

  long checksumP1 = calcChecksum(blocksP1);
  std::cout << "Part 1 Checksum: " << checksumP1 << std::endl;

  long checksumP2 = calcChecksum(blocksP2);
  std::cout << "Part 2 Checksum: " << checksumP2 << std::endl;

  exit(EXIT_SUCCESS);
}