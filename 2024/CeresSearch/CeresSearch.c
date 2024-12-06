#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include <ctype.h>

char* left_word(int row_len, int col_len, char board[row_len][col_len], int row, int col) {
  char* word = malloc(4 * sizeof(char));

  if (col < 3) {
    return NULL;
  }

  for (int i = 0; i < 4; ++i) {
    word[i] = board[row][col - i];
  }

  return word;
}

char* right_word(int row_len, int col_len, char board[row_len][col_len], int row, int col) {
  char* word = malloc(4 * sizeof(char));

  if (col > col_len - 4) {
    return NULL;
  }

  for (int i = 0; i < 4; ++i) {
    word[i] = board[row][col + i];
  }

  return word;
}

char* up_word(int row_len, int col_len, char board[row_len][col_len], int row, int col) {
  char* word = malloc(4 * sizeof(char));

  if (row < 3) {
    return NULL;
  }

  for (int i = 0; i < 4; ++i) {
    word[i] = board[row - i][col];
  }

  return word;
}

char* down_word(int row_len, int col_len, char board[row_len][col_len], int row, int col) {
  char* word = malloc(4 * sizeof(char));

  if (row > row_len - 4) {
    return NULL;
  }

  for (int i = 0; i < 4; ++i) {
    word[i] = board[row + i][col];
  }

  return word;
}

char* up_left_word(int row_len, int col_len, char board[row_len][col_len], int row, int col) {
  char* word = malloc(4 * sizeof(char));

  if (col < 3 || row < 3) {
    return NULL;
  }

  for (int i = 0; i < 4; ++i) {
    word[i] = board[row - i][col - i];
  }

  return word;
}

char* up_right_word(int row_len, int col_len, char board[row_len][col_len], int row, int col) {
  char* word = malloc(4 * sizeof(char));

  if (row < 3 || col > col_len - 4) {
    return NULL;
  }

  for (int i = 0; i < 4; ++i) {
    word[i] = board[row - i][col + i];
  }

  return word;
}

char* down_left_word(int row_len, int col_len, char board[row_len][col_len], int row, int col) {
  char* word = malloc(4 * sizeof(char));

  if (row > row_len - 4 || col < 3) {
    return NULL;
  }

  for (int i = 0; i < 4; ++i) {
    word[i] = board[row + i][col - i];
  }

  return word;
}

char* down_right_word(int row_len, int col_len, char board[row_len][col_len], int row, int col) {
  char* word = malloc(4 * sizeof(char));
  
  if (row > row_len - 4 || col > col_len - 4) {
    return NULL;
  }

  for (int i = 0; i < 4; ++i) {
    word[i] = board[row + i][col + i];
  }

  return word;
}

// Checks for XMAS but only starts at X values
int occurences(int row_len, int col_len, char board[row_len][col_len], int row, int col) {
  if (row >= row_len || col >= col_len || board[row][col] != 'X') {
    return 0;
  }

  int count = 0;
  char** words = malloc(8 * sizeof(char*));

  words[0] = left_word(row_len, col_len, board, row, col);
  words[1] = right_word(row_len, col_len, board, row, col);
  words[2] = up_word(row_len, col_len, board, row, col);
  words[3] = down_word(row_len, col_len, board, row, col);
  words[4] = up_left_word(row_len, col_len, board, row, col);
  words[5] = up_right_word(row_len, col_len, board, row, col);
  words[6] = down_left_word(row_len, col_len, board, row, col);
  words[7] = down_right_word(row_len, col_len, board, row, col);

  for (int i = 0; i < 8; ++i) {
    if (words[i] == NULL) {
      continue;
    }

    bool valid = true;

    if (words[i][0] != 'X') {
      valid = false;
    } else if (words[i][1] != 'M') {
      valid = false;
    } else if (words[i][2] != 'A') {
      valid = false;
    } else if (words[i][3] != 'S') {
      valid = false;
    }

    count += valid;
  }

  return count;
}

bool isXmas(int row_len, int col_len, char board[row_len][col_len], int row, int col) {
  if (row < 1 || col < 1 || row > row_len - 2 || col > col_len - 2 || board[row][col] != 'A') {
    return false;
  }

  bool valid = true;
  char top_left = board[row - 1][col - 1];
  char top_right = board[row - 1][col + 1];
  char bottom_left = board[row + 1][col - 1];
  char bottom_right = board[row + 1][col + 1];

  if (top_left != 'S' && top_left != 'M') {
    valid = false;
  } 
  if (top_right != 'S' && top_right != 'M') {
    valid = false;
  }
  if (bottom_left != 'S' && bottom_left != 'M') {
    valid = false;
  }
  if (bottom_right != 'S' && bottom_right != 'M') {
    valid = false;
  }

  if (top_left == bottom_right) {
    valid = false;
  }
  if (bottom_left == top_right) {
    valid = false;
  }

  return valid;
}

int main(void) {
  const char* fname = "input.txt";

  FILE* fp = fopen(fname, "r");
  if (!fp) {
    perror("Unable to open file");
    exit(EXIT_FAILURE);
  }

  // get puzzle dimensions
  char* line = NULL;
  size_t row = 0;
  size_t col = 0;
  size_t line_len = 0;
  size_t pos = getline(&line, &line_len, fp);

  for (size_t i = 0; i < strlen(line); ++i) {
    if (isalnum(line[i])) {
      ++col;
    }
  }

  while(pos != EOF) {
    ++row;
    pos = getline(&line, &line_len, fp);
  }

  rewind(fp);

  char board[row][col];
  int i = 0;
  int j = 0;
  char c;
  while (((c = fgetc(fp)) != EOF)) {
    if (isalnum(c)) {
      board[i][j % col] = c;

      ++j;
      if (j == col) {
        j = 0;
        ++i;
      }
    }
  }

  // Part 1
  int total = 0;

  for (int i = 0; i < row; ++i) {
    for (int j  = 0; j < col; ++j) {
      total += occurences(row, col, board, i, j);
    }
  }

  printf("\nTotal: %d\n", total);

  // Part 2
  int totalXmas = 0;

  for (int i = 0; i < row; ++i) {
    for (int j = 0; j < col; ++j) {
      totalXmas += isXmas(row, col, board, i, j) ? 1 : 0;
    }
  }

  printf("\nTotal Xmas: %d\n", totalXmas);

}