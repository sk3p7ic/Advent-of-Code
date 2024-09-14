#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#define N_ROWS 300
#define N_COLS 33

void panic(const char* msg);
void expect(void* ptr, const char* msg);

void readFile(const char* filename, char* buf);


typedef enum { CT_ON, CT_OFF, CT_TOG } CommandType;

typedef struct {
    int x;
    int y;
} Coord;

typedef struct {
    CommandType ct;
    Coord c1;
    Coord c2;
} Command;

Command parseLine(const char* line) {
    Command cmd = {};
    // Parse out "turn" string prefix, if present
    char turnStr[] = "turn";
    char* input = malloc(strlen(line));
    expect(input, "Could not allocate input buffer to parse line to Command.");
    if (!strncmp(line, turnStr, strlen(turnStr))) {
        strcpy(input, &line[strlen(turnStr) + 1]);
    } else {
        strcpy(input, line);
    }

    // Define vars to store sccanf results
    char state[8];
    Coord c1 = {};
    Coord c2 = {};

    // Parse the line
    sscanf(input, "%s %d,%d through %d,%d", state, &c1.x, &c1.y, &c2.x, &c2.y);

    // Not the "cleanest" way to parse, but it works easily for this case.
    switch (state[1]) {
        case 'n': cmd.ct = CT_ON; break;
        case 'f': cmd.ct = CT_OFF; break;
        case 'o': cmd.ct = CT_TOG;
    }

    cmd.c1 = c1;
    cmd.c2 = c2;

    free(input);

    return cmd;
}

int max(int a, int b) {
    return (a > b) ? a : b;
}

int processP1(Command* cmds, size_t n_commands) {
    short grid[1000][1000] = {0};

    for (size_t i = 0; i < n_commands; ++i) {
        Command cmd = cmds[i];
        for (int r = cmd.c1.y; r <= cmd.c2.y; ++r) {
            for (int c = cmd.c1.x; c <= cmd.c2.x; ++c) {
                switch (cmd.ct) {
                    case CT_ON:
                        grid[r][c] = 1;
                        break;
                    case CT_OFF:
                        grid[r][c] = 0;
                        break;
                    case CT_TOG:
                        grid[r][c] = !grid[r][c];
                }
            }
        }
    }

    int nLit = 0;
    for (size_t i = 0; i < 1000; ++i) {
        for (size_t j = 0; j < 1000; ++j) {
            if (grid[i][j]) nLit++;
        }
    }
    return nLit;
}

int processP2(Command* cmds, size_t n_commands) {
    short grid[1000][1000] = {0};

    for (size_t i = 0; i < n_commands; ++i) {
        Command cmd = cmds[i];
        for (int r = cmd.c1.y; r <= cmd.c2.y; ++r) {
            for (int c = cmd.c1.x; c <= cmd.c2.x; ++c) {
                switch (cmd.ct) {
                    case CT_ON:
                        grid[r][c] += 1;
                        break;
                    case CT_OFF:
                        grid[r][c] = max(grid[r][c] - 1, 0);
                        break;
                    case CT_TOG:
                        grid[r][c] += 2;
                }
            }
        }
    }

    int brightness = 0;
    for (size_t i = 0; i < 1000; ++i) {
        for (size_t j = 0; j < 1000; ++j) {
            brightness += grid[i][j];
        }
    }
    return brightness;
}

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Incorrect number of arguments given (%d).\n", argc);
        fprintf(stderr, "Usage: day06 <input-file>\n");
        exit(EXIT_FAILURE);
    }
    char* buf = malloc(N_ROWS * N_COLS);
    expect(buf, "Could not allocate buffer to store input file contents.");
    readFile(argv[1], buf);

    Command* cmds = malloc(sizeof(Command) * N_ROWS);
    expect(cmds, "Could not allocate buffer to store commands.");

    char* line = malloc(N_COLS);
    expect(line, "Could not allocate buffer to read input file lines.");
    line = strtok(buf, "\n");
    for (size_t i = 0; i < N_ROWS; ++i) {
        cmds[i] = parseLine(line);
        line = strtok(NULL, "\n");
        if (line == NULL) {
            fprintf(stderr, "Got NULL line when not expected (idx=%ld)\n", i);
            break;
        }
    }
    free(buf);

    printf("D06P01: %d\n", processP1(cmds, N_ROWS));
    printf("D06P02: %d\n", processP2(cmds, N_ROWS));
    
    free(cmds);
    return 0;
}

void panic(const char* msg) {
    perror(msg);
    exit(EXIT_FAILURE);
}

void expect(void* ptr, const char* msg) {
    if (ptr == NULL) panic(msg);
}

void readFile(const char* filename, char* buf) {
    FILE* f = fopen(filename, "r");
    expect(f, "Could not open input file.");

    size_t idx = 0;
    while (!feof(f)) {
        buf[idx++] = fgetc(f);
    }

    fclose(f);
}
