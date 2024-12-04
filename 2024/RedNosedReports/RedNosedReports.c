#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>

int* processLine(char* line, size_t len, size_t* out_len) {
  int* result = malloc(len * sizeof(int));
  if (result == NULL) {
    perror("Unable to allocate memory");
    exit(EXIT_FAILURE);
  }

  char* num = strtok(line, " ");
  int i = 0;
  while(num != NULL) {
    result[i] = strtol(num, NULL, 10);
    ++i;
    num = strtok(NULL, " ");
  }

  *out_len = i;
  return result;
}

bool isValidAscending(int* report, size_t len) {
  bool result = true;

  int prev = report[0];
  for (int i = 1; i < len; ++i) {
    int diff = abs(report[i] - prev);

    if (diff < 1 || diff > 3 || report[i] < prev) {
      result = false;
      break;
    }

    prev = report[i];
  }

  return result;
}

bool isValidDescending(int* report, size_t len) {
  bool result = true;

  int prev = report[0];
  for (int i = 1; i < len; ++i) {
    int diff = abs(report[i] - prev);

    if (diff < 1 || diff > 3 || report[i] > prev) {
      result = false;
      break;
    }

    prev = report[i];
  }

  return result;
}

bool isSafe(int* report, size_t len) {
  if (isValidAscending(report, len) || isValidDescending(report, len)) {
    return true;
  }

  for (int i = 0; i < len; ++i) {
    int* sub_report = malloc(len * sizeof(int) - 1);
    int count = 0;
    for (int j = 0; j < len; ++j) {
      if (j != i) {
        sub_report[count] = report[j];
        count++;
      }
    }

    if (isValidAscending(sub_report, len - 1) || isValidDescending(sub_report, len - 1)) {
      free(sub_report);
      return true;
    }

    free(sub_report);
  }

  return false;
}

int main(void) {
  const char* fname = "input.txt";

  FILE* fp = fopen(fname, "r");
  if (!fp) {
    perror("Unable to open file");
    return 1;
  }

  rewind(fp);

  int line_count = 0;
  int safe_count = 0;
  char* line = NULL;
  size_t line_len = 0;
  size_t pos = getline(&line, &line_len, fp);
  while(pos != EOF) {
    size_t rep_len = 0;
    int* report = processLine(line, line_len, &rep_len);

    if (isSafe(report, rep_len)) {
      ++safe_count;
    }

    free(report);
    pos = getline(&line, &line_len, fp);
  }

  free(line);
  fclose(fp);

  printf("Safe count: %d\n", safe_count);
}