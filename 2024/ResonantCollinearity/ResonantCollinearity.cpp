#include <iostream>
#include <string>
#include <fstream>
#include <vector>
#include <map>
#include <set>

struct vec2 {
  int x;
  int y;
};

void printMap(std::map<char, std::vector<vec2>> map) {
  for (auto iter = map.begin(); iter != map.end(); ++iter) {
    std::cout << iter->first << ": ";
    for (vec2 pos : iter->second) {
      std::cout << "(" << pos.x << ", " << pos.y << ") ";
    }
    std::cout << std::endl;
  }
}

std::vector<vec2> getAntinodes(std::vector<vec2> positions, int xMax, int yMax) {
  std::vector<vec2> result;

  for (int i = 0; i < positions.size(); ++i) {
    for (int j = i + 1; j < positions.size(); ++j) {
      vec2 pos1 = positions[i];
      vec2 pos2 = positions[j];
      vec2 diff = vec2 { pos2.x - pos1.x, pos2.y - pos1.y };

      vec2 nodePos1 = vec2 { pos1.x - diff.x, pos1.y - diff.y };
      vec2 nodePos2 = vec2 { pos2.x + diff.x, pos2.y + diff.y };

      while (nodePos1.x > -1 && nodePos1.x <= xMax && nodePos1.y > -1 && nodePos1.y <= yMax) {
        result.push_back(vec2 { nodePos1.x, nodePos1.y });
        nodePos1.x -= diff.x;
        nodePos1.y -= diff.y;
      }

      while (nodePos2.x > -1 && nodePos2.x <= xMax && nodePos2.y > -1 && nodePos2.y <= yMax) {
        result.push_back(vec2 { nodePos2.x, nodePos2.y });
        nodePos2.x += diff.x;
        nodePos2.y += diff.y;
      }

      result.push_back(pos1);
      result.push_back(pos2);
    }
  }


  return result;
}

std::vector<vec2> filterNodes(std::vector<vec2> positions, int xMax, int yMax) {
  std::set<std::string> posSet;
  std::vector<vec2> result;

  for (vec2 pos : positions) {
    bool valid = true;

    if (pos.x < 0 || pos.x > xMax || pos.y < 0 || pos.y > yMax) {
      valid = false;
    }

    std::string posStr = std::to_string(pos.x) + "," + std::to_string(pos.y);
    if (posSet.find(posStr) != posSet.end()) {
      valid = false;
    }

    posSet.insert(posStr);
    if (valid) {
      result.push_back(pos);
    }
  }

  return result;
}

int main(int argc, char* argv[]) {
  bool debug = false;
  bool sample = false;

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

  std::string fname = sample ? "sampleinput.txt" : "input.txt";
  std::ifstream input(fname);

  if (!input.is_open()) {
    std::cerr << "ERROR Could not open file" << std::endl;
    exit(EXIT_FAILURE);
  }

  std::string line;
  std::map<char, std::vector<vec2>> map;
  int lineCount = 0;
  int xMax = 0;
  int yMax = 0;
  while (getline(input, line)) {
    xMax = std::max(xMax, (int) line.length() - 1);

    for (int i = 0; i < line.length(); ++i) {
      if (line[i] == '.') {
        continue;
      }

      auto iter = map.find(line[i]);
      if (iter != map.end()) {
        vec2 pos = vec2 { i, lineCount };
        iter->second.push_back(pos);
      } else {
        vec2 pos = vec2 { i, lineCount };
        std::vector<vec2> vec { pos };
        map[line[i]] = vec;
      }
    }

    ++lineCount;
  }
  yMax = lineCount - 1;

  if (debug) {
    std::cout << "Max X: " << xMax << " | Max Y: " << yMax << std::endl;
    std::cout << "Map" << std::endl;
    printMap(map);
  }

  std::vector<vec2> nodes;
  for (auto iter = map.begin(); iter != map.end(); ++iter) {
    std::vector<vec2> vec = getAntinodes(iter->second, xMax, yMax);
    nodes.insert(nodes.end(), vec.begin(), vec.end());
  }

  std::vector<vec2> filteredNodes = filterNodes(nodes, xMax, yMax);
  std::cout << "Antinode Count: " << filteredNodes.size() << std::endl;

  exit(EXIT_SUCCESS);
}