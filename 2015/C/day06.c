#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#define N_ROWS 300
#define N_COLS 34
#define GRID_SZ 1000 * 1000

void panic(const char* msg);
void expect(void* ptr, const char* msg);

void readFile(const char* filename, char buf[N_ROWS][N_COLS]);


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

Coord parseCoord(const char* str) {
    // Create dynamic input buffer
    char* input = malloc(strlen(str));
    expect(input, "Cannot create buffer to parse coordinate.");
    strcpy(input, str);

    // Parse the input
    Coord c;
    char *tok = malloc(3);
    expect(tok, "Cannot create coordinate parsing token buffer.");

    tok = strtok(input, ",");
    expect(tok, "Coordinate 'x' cannot be NULL.");
    c.x = atoi(tok);
    tok = strtok(NULL, ",");
    expect(tok, "Coordinate 'y' cannot be NULL.");
    c.y = atoi(tok);

    free(input);

    return c;
}

Command parseLine(const char* line) {
    // Create a dynamic input
    char* input = malloc(strlen(line));
    expect(input, "Cannot create buffer to parse input line.");
    strcpy(input, line);
    
    // Parse the input
    Command cmd = {};
    char* tok = malloc(7);
    expect(tok, "Cannot create token buffer.");
    char* c1Str = malloc(7);
    char* c2Str = malloc(7);
    expect(c1Str, "Cannot create buffer for Coordinate 1 string.");
    expect(c1Str, "Cannot create buffer for Coordinate 2 string.");

    int parseC2 = 0;

    tok = strtok(input, " ");
    while (tok != NULL) {
        int foundCommandToken = 0;
        if (!strcmp(tok, "turn")) foundCommandToken = 1; // Ignore "turn" token
        if (!strcmp(tok, "on")) { cmd.ct = CT_ON; foundCommandToken = 1; }
        if (!strcmp(tok, "off")) { cmd.ct = CT_OFF; foundCommandToken = 1; }
        if (!strcmp(tok, "toggle")) { cmd.ct = CT_TOG; foundCommandToken = 1; }
        if (!strcmp(tok, "through")) { parseC2 = 1; foundCommandToken = 1; }

        if (!foundCommandToken) {
            if (!parseC2) strcpy(c1Str, tok);
            else strcpy(c2Str, tok);
        }

        tok = strtok(NULL, " ");
    }

    cmd.c1 = parseCoord(c1Str);
    cmd.c2 = parseCoord(c2Str);

    free(tok);
    free(c1Str);
    free(c2Str);
    free(input);

    return cmd;
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
                    case CT_OFF:
                        grid[r][c] = 0;
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

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Incorrect number of arguments given (%d).\n", argc);
        fprintf(stderr, "Usage: day06 <input-file>\n");
        exit(EXIT_FAILURE);
    }
    char buf[N_ROWS][N_COLS];
    readFile(argv[1], buf);

    Command* cmds = malloc(sizeof(Command) * N_ROWS);
    expect(cmds, "Could not allocate buffer to store commands.");
    for (size_t i = 0; i < N_ROWS; ++i) {
        cmds[i] = parseLine(buf[i]);
    }

    printf("D06P01: %d\n", processP1(cmds, N_ROWS));
    
    Command cmdTest[] = { parseLine("turn on 0,0 through 999,999") };
    printf("cmdTest: %d\n", processP1(cmdTest, 1));

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

void readFile(const char* filename, char buf[N_ROWS][N_COLS]) {
    FILE* f = fopen(filename, "r");
    expect(f, "Could not open input file.");

    size_t row = 0;
    size_t col = 0;

    while (!feof(f) && row < N_ROWS) {
        char c = fgetc(f);
        if (c == '\r') continue;
        if (c == '\n') {
            col = 0;
            row++;
            continue;
        }
        buf[row][col++] = c;
    }

    fclose(f);
}
