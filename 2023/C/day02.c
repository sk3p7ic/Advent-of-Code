#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N_COLORS 3
#define N_LINES 1024
#define LINE_LEN 254

#define max_n(x,y) (((x) >= (y)) ? (x) : (y))

struct GameStat {
    int r;
    int g;
    int b;
};

struct Color {
    char *str;
    enum {Red, Green, Blue} val;
};

const struct Color COLORS[N_COLORS] = {
    {"red", Red}, {"green", Green}, {"blue", Blue}
};

struct GameStat max_for_game(char *game) {
    struct GameStat maxes = {0, 0, 0};
    char *token = strtok(game, " ");
    int pos = 0;
    int value = 0;
    while (token != NULL) {
        if (pos++ % 2 == 0) {
            value = atoi(token);
        } else {
            for (int i = 0; i < N_COLORS; i++) {
                const struct Color c = COLORS[i];
                if (!strncmp(token, c.str, strlen(c.str))) {
                    switch (c.val) {
                        case Red: maxes.r = max_n(maxes.r, value); break;
                        case Green: maxes.g = max_n(maxes.g, value); break;
                        case Blue: maxes.b = max_n(maxes.b, value); break;
                    }
                    break;
                }
            }
        }
        token = strtok(NULL, " ");
    }
    return maxes;
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
    char *games[n_lines_read];
    for (int i = 0; i < n_lines_read; i++) {
        char *line = contents[i];
        char *token = strtok(line, ":");
        char *game;
        while (token != NULL) {
            game = token;
            token = strtok(NULL, ":");
        }
        games[i] = game;
    }
    struct GameStat max_ns_for_p1 = {12, 13, 14};
    struct GameStat stat;
    int good_games = 0;
    int fewest = 0;
    for (int i = 0; i < n_lines_read; i++) {
        stat = max_for_game(games[i]);
        if (stat.r <= max_ns_for_p1.r && stat.g <= max_ns_for_p1.g &&
            stat.b <= max_ns_for_p1.b) {
            good_games += i + 1;
        }
        fewest += stat.r * stat.b * stat.g;
    }
    printf("[2023] D02P01: %d\n", good_games);
    printf("[2023] D02P02: %d\n", fewest);
}
