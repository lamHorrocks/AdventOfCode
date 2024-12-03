#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int compare(const void* a, const void* b) {
  int int_a = *((int*) a);
  int int_b = *((int*) b);

  if (int_a == int_b) {
    return 0;
  } else if (int_a < int_b) {
    return -1;
  }

  return 1;
}

int main(void) {
  const char* fname = "input.txt";

  FILE* fp = fopen(fname, "r");
  if (!fp) {
    perror("Unable to open file");
    return 1;
  }

  int line_count = 0;
  char* line = NULL;
  size_t line_len = 0;
  size_t read = getline(&line, &line_len, fp);
  while ((getline(&line, &line_len, fp)) != EOF) {
    ++line_count;
  }

  rewind(fp);

  int list1[line_count + 1];
  int list2[line_count + 1];
  line_count = 0;
  line = NULL;
  line_len = 0;
  read = getline(&line, &line_len, fp);
  while (read != EOF) {
    if (line != NULL) {
      char* num1 = strtok(line, " ");
      char* num2 = strtok(NULL, " ");

      list1[line_count] = strtol(num1, NULL, 10);
      list2[line_count] = strtol(num2, NULL, 10);
    }

    read = getline(&line, &line_len, fp);
    ++line_count;
  }

  qsort(list1, line_count, sizeof(int), compare);
  qsort(list2, line_count, sizeof(int), compare);

  int sum = 0;
  for (int i = 0; i < line_count; ++i) {
    sum += abs(list1[i] - list2[i]);
  }

  free(line);
  fclose(fp);

  printf("SUM: %d\n", sum);

  return 0;
}
