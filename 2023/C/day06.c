#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

#define N_LINES 2
#define LINE_LEN 64
#define N_NUMS 4

#define max_n(x,y) (((x) >= (y)) ? (x) : (y))

void to_numbers(char *data, int *nums, int n_nums) {
    int line_ptr = 0;
    for (int n = 0; n < n_nums; n++) {
        char buf[16] = {0};
        int buf_ptr = 0;
        while (line_ptr < strlen(data) && !isdigit(data[line_ptr])) {
            line_ptr++;
        }
        while (line_ptr < strlen(data) && isdigit(data[line_ptr])) {
            buf[buf_ptr++] = data[line_ptr++];
        }
        nums[n] = atoi(buf);
    }
}

long int to_number(char *data) {
    int n_digits = 0;
    long int num = 0;
    int ptr = 0;
    while (ptr < strlen(data)) {
        while (ptr < strlen(data) && isspace(data[ptr])) { ptr++; }
        while (ptr < strlen(data) && isdigit(data[ptr])) {
            num = (num * n_digits++ * 10) + (data[ptr++] - '0');
            printf("%ld\n", num);
        }
    }
    return num;
}

int gen_starts(int *times, int n_times, int *target) {
    int n_wins = 1; // We know we're guaranteed at least one win
    for (int i = 0; i < n_times; i++) {
        int n_round_wins = 0;
        for (int t = 1; t < times[i]; t++) {
            if (t * (times[i] - t) > target[i]) n_round_wins++;
        }
        n_wins *= n_round_wins; // We know a round will have at least 1 win
    }
    return n_wins;
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
    char contents[N_LINES][LINE_LEN];
    int n_lines_read = 0;
    while (fgets(contents[n_lines_read], LINE_LEN, fp)) {
        contents[n_lines_read][strlen(contents[n_lines_read]) - 1] = '\0';
        n_lines_read++;
    }
    fclose(fp);
    int split_nums[N_LINES][N_NUMS];
    long int singl_nums[N_LINES][1];
    for (int i = 0; i < N_LINES; i++) {
        char *line = contents[i];
        char *token = strtok(line, ":");
        char *d;
        while (token != NULL) {
            d = token;
            token = strtok(NULL, ":");
        }
        int line_nums[N_NUMS];
        to_numbers(d, line_nums, N_NUMS);
        memcpy(split_nums[i], line_nums, sizeof(int) * N_NUMS);
        //to_numbers(d, singl_nums[i], 1);
        long int v[1] = {to_number(d)};
        memcpy(singl_nums[i], v, sizeof(long int));
    }
    for (int i = 0; i < N_LINES; i++) {
        printf("%d, %d, %d, %d\n", split_nums[i][0], split_nums[i][1], split_nums[i][2], split_nums[i][3]);
        printf("%ld\n", singl_nums[i][0]);
    }
    printf("Wins: %d\n", gen_starts(split_nums[0], N_NUMS, split_nums[1]));
    //printf("Wins: %d\n", gen_starts(singl_nums[0], 1, singl_nums[1]));
    //printf("[2023] D02P01: %d\n", good_games);
    //printf("[2023] D02P02: %d\n", fewest);
}
