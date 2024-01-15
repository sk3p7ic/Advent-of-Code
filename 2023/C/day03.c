#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N_LINES 1024
#define LINE_LEN 254

#define max_n(x, y) (x >= y ? (x) : (y))

typedef struct {
    int r;
    int c;
} Coord;

typedef struct {
    Coord c;
    char v;
} MtxChr;

typedef struct {
    Coord c;
    int v;
    MtxChr *assoc_chars[];
} MtxNum;

typedef union {
    MtxChr chr;
    MtxNum num;
} MtxVal;

int main(int argc, char **argv) {
    if (argc != 2) {
        fprintf(stderr, "Incorrect number of arguments! (Got %d)\n", argc);
        fprintf(stderr, "Format: day03 <filename>\n");
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
    int max_line_len = 0;
    while (fgets(contents[n_lines_read], LINE_LEN, fp)) {
        int llen = strlen(contents[n_lines_read]);
        if (strchr(contents[n_lines_read], '\n') != NULL) { // Ignore newlines
            llen--;
        }
        contents[n_lines_read][llen] = '\0';
        max_line_len = max_n(max_line_len, llen);
        n_lines_read++;
    }
    fclose(fp);
    MtxVal mtx[n_lines_read][max_line_len];
    int r, c = 0;
    while (r < n_lines_read) {
        while (c < max_line_len) {
            char curr = contents[r][c];
            MtxVal mv;
            if (!isdigit(curr)) {
                MtxChr mc = {{r, c}, curr};
                mv.chr = mc;
                c++;
            } else {
                char *d;
                int digit_len = 0;
                printf("\nFound a digit\n");
                while (isdigit(contents[r][c]) && c < max_line_len) {
                    printf("%c", contents[r][c]);
                    char *nd = malloc(sizeof(char) * (digit_len + 1));
                    memcpy(nd, d, digit_len);
                    nd[digit_len] = contents[r][c++];
                    d = nd;
                    digit_len++;
                }
                MtxNum mn = {{r, c}, atoi(d), {}};
                mv.num = mn;
            }
            mtx[r][c] = mv;
        }
        r++;
    }
    //printf("[2023] D02P01: %d\n", good_games);
    //printf("[2023] D02P02: %d\n", fewest);
}
