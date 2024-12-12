#include <vector>
#include <fstream>
#include <iostream>
#include <string>
#include <cctype>
#include <set>

enum Direction { UP, DOWN, LEFT, RIGHT };

struct Tile {
  bool isObstacle;
  std::set<Direction> history;
};

void printBoard(const std::vector<std::vector<Tile>>& board) {
  for (std::vector<Tile> line : board) {
    for (Tile tile : line) {
      if (tile.isObstacle) {
        std::cout << '#';
      } else if (!tile.history.empty()) {
        std::cout << 'X';
      } else {
        std::cout << '.';
      }
    }
    std::cout << std::endl;
  }
}

void clearBoardHistory(std::vector<std::vector<Tile>>& board) {
  for (std::vector<Tile>& line : board) {
    for (Tile& tile : line) {
      tile.history.clear();
    }
  }
}

int countTraversedTiles(const std::vector<std::vector<Tile>>& board) {
  int count = 0;

  for (std::vector<Tile> line : board) {
    for (Tile tile : line) {
      count += tile.history.empty() ? 0 : 1;
    }
  }

  return count;
}

Direction getNextDirection(Direction dir) {
  Direction next = dir;

  switch (dir) {
    case Direction::UP:
      next = Direction::RIGHT;
      break;
    case Direction::RIGHT:
      next = Direction::DOWN;
      break;
    case Direction::DOWN:
      next = Direction::LEFT;
      break;
    case Direction::LEFT:
      next = Direction::UP;
      break;
  }

  return next;
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
  std::vector<std::vector<Tile>> board;
  int startRow = -1;
  int startCol = -1;
  while (getline(input, line)) {
    std::vector<Tile> tileLine;

    for (char c : line) {
      Tile tile;
      tile.isObstacle = false;
      tile.history = std::set<Direction>();

      if (c == '#') {
        tile.isObstacle = true;
      }

      if (c == '^') {
        startRow = board.size();
        startCol = tileLine.size();
      }

      tileLine.push_back(tile);
    }

    board.push_back(tileLine);
  }

  if (board.size() == 0) {
    std::cerr << "ERROR Empty Board" << std::endl;
    exit(EXIT_FAILURE);
  }

  if (debug) {
    std::cout << "Initial Board" << std::endl;
    printBoard(board);
    std::cout << std::endl;
  }

  int maxRow = board.size();
  int maxCol = board[0].size();

  if (startRow == -1 || startCol == -1) {
    std::cerr << "ERROR Unable to retrieve start position" << std::endl;
    exit(EXIT_FAILURE);
  }

  int row = startRow;
  int col = startCol;
  Direction dir = Direction::UP;
  while(row > -1 && col > -1 && row < maxRow && col < maxCol) {
    board[row][col].history.insert(dir);

    int nextRow = -1;
    int nextCol = -1;

    switch (dir) {
      case Direction::UP:
        nextRow = row - 1;
        nextCol = col;
        break;
      case Direction::RIGHT:
        nextRow = row;
        nextCol = col + 1;
        break;
      case Direction::DOWN:
        nextRow = row + 1;
        nextCol = col;
        break;
      case Direction::LEFT:
        nextRow = row;
        nextCol = col - 1;
        break;
    }

    bool nextIsObstacle = nextRow > -1 && nextCol > -1 && nextRow < maxRow && nextCol < maxCol && board[nextRow][nextCol].isObstacle;
    if (nextIsObstacle) {
      dir = getNextDirection(dir);
    } else {
      row = nextRow;
      col = nextCol;
    }
  }

  if (debug) {
    std::cout << "End Board" << std::endl;
    printBoard(board);
    std::cout << std::endl;
  }

  int count = countTraversedTiles(board);
  std::cout << "Part 1: " << count << std::endl;

  // Part 2 - brute force, my beloved
  int obstructionCount = 0;
  for (int i = 0; i < board.size(); ++i) {
    for (int j = 0; j < board.size(); ++j) {
      clearBoardHistory(board);
      bool removeObstacle = !board[i][j].isObstacle;
      board[i][j].isObstacle = true;
      int row = startRow;
      int col = startCol;
      Direction dir = Direction::UP;

      while(row > -1 && col > -1 && row < maxRow && col < maxCol) {
        board[row][col].history.insert(dir);

        int nextRow = -1;
        int nextCol = -1;

        switch (dir) {
          case Direction::UP:
            nextRow = row - 1;
            nextCol = col;
            break;
          case Direction::RIGHT:
            nextRow = row;
            nextCol = col + 1;
            break;
          case Direction::DOWN:
            nextRow = row + 1;
            nextCol = col;
            break;
          case Direction::LEFT:
            nextRow = row;
            nextCol = col - 1;
            break;
        }

        if (nextRow > -1 && nextCol > -1 && nextRow < maxRow && nextCol < maxCol) {
          Tile nextTile = board[nextRow][nextCol];

          if (nextTile.isObstacle) {
            dir = getNextDirection(dir);
            continue;
          } else {
            if (nextTile.history.find(dir) != nextTile.history.end()) {
              ++obstructionCount;
              break;
            }

          }
        }

        row = nextRow;
        col = nextCol;
      }

      if (removeObstacle) {
        board[i][j].isObstacle = false;
      }
    }
  }

  std::cout << "Part 2: " << obstructionCount << std::endl;

  exit(EXIT_SUCCESS);
}