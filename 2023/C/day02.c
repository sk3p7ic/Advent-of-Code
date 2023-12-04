#include <assert.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

#define N_COLORS 3
#define N_LINES 1024
#define LINE_LEN 254

#define min_n(x,y) (((x) <= (y)) ? (x) : (y))

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

struct GameStat max_for_game_rounds(char *game) {
    struct GameStat maxes = {1024, 1024, 1024};
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
                        case Red: maxes.r = min_n(value, maxes.r); break;
                        case Green: maxes.g = min_n(value, maxes.g); break;
                        case Blue: maxes.b = min_n(value, maxes.b); break;
                    }
                    // Whoops, this way won't work because it ignores the
                    // rounds... I'll fix that at a later date.
                }
            }
        }
        token = strtok(NULL, " ");
    }
    return maxes;
}

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
                        case Red: maxes.r += value; break;
                        case Green: maxes.g += value; break;
                        case Blue: maxes.b += value; break;
                    }
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
    struct GameStat stats[n_lines_read];
    int good_games = 0;
    for (int i = 0; i < n_lines_read; i++) {
        stats[i] = max_for_game_rounds(games[i]);
        printf("r: %d, g: %d, b: %d\n", stats[i].r, stats[i].g, stats[i].b);
        if (stats[i].r <= max_ns_for_p1.r && stats[i].g <= max_ns_for_p1.g &&
            stats[i].b <= max_ns_for_p1.b) {
            good_games += i + 1;
        }
    }
    printf("[2023] D02P01: %d\n", good_games);
    //printf("[2032] D01P01: %d\n", sum_lines(contents, false));
    //printf("[2032] D01P02: %d\n", sum_lines(contents, true));
}
