#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

#define N_INPUT_LINES 1000
#define INPUT_LINE_LEN 254

int find_digit_in_line(char *line) {
    char *digits[10] = {"zero", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine"};
    for (int i = 0; i < 10; i++) {
        char *needle = digits[i];
        if (!strncmp(line, needle, strlen(needle))) {
            return i;
        }
    }
    return -1;
}

int parse_line(char *line, bool replace) {
    int first = -1;
    int last = -1;
    for (int i = 0; i < strlen(line); i++) {
        if (isdigit(line[i])) {
            if (first == -1) {
                first = line[i] - '0';
                last = first;
            } else {
                last = line[i] - '0';
            }
            continue;
        }
        if (replace) {
            int digit = find_digit_in_line(&line[i]);
            if (digit != -1) {
                if (first == -1) {
                    first = digit;
                    last = first;
                } else {
                    last = digit;
                }
            }
        }
    }
    return first * 10 + last;
}

int sum_lines(char lines[N_INPUT_LINES][INPUT_LINE_LEN], bool replace) {
    int sum = 0;
    for (int i = 0; i < N_INPUT_LINES; i++) {
        sum += parse_line(lines[i], replace);
    }
    return sum;
}

int main(int argc, char **argv) {
    if (argc != 2) {
        fprintf(stderr, "Incorrect number of arguments! (Got %d)\n", argc);
        fprintf(stderr, "Format: day01 <filename>\n");
        return 1;
    }
    char *fname = argv[1];
    FILE *fp = fopen(fname, "r");
    if (fp == NULL) {
        fprintf(stderr, "Error opening file '%s'\n", fname);
        return 1;
    }
    char contents[N_INPUT_LINES][INPUT_LINE_LEN];
    {
        int i = 0;
        while (fgets(contents[i], INPUT_LINE_LEN, fp)) {
            contents[i][strlen(contents[i]) - 1] = '\0';
            i++;
        }
    }
    fclose(fp);
    printf("[2032] D01P01: %d\n", sum_lines(contents, false));
    printf("[2032] D01P02: %d\n", sum_lines(contents, true));
}
