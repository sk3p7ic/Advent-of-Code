#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N_INPUT_CHARS 9000

typedef struct {
    int x;
    int y;
} Coord;

typedef struct {
    Coord *items;
    size_t maxItems;
    size_t nItems;
} CoordSet;

CoordSet csInit(size_t baseLength) {
    Coord *items = malloc(baseLength * sizeof(Coord));
    if (items == NULL) {
        perror("Could not allocate a new CoordSet.");
        exit(EXIT_FAILURE);
    }
    CoordSet cs = { items, baseLength };
    return cs;
}

void csAppend(CoordSet *cs, Coord c) {
    if (cs->nItems > 0) {
        // TODO: Implement hashing to avoid O(n) search
        for (size_t i = 0; i < cs->nItems; ++i) {
            Coord coord = cs->items[i];
            if (coord.x == c.x && coord.y == c.y) return;
        }
    }
    // Expand chunk of memory if needed
    if (cs->nItems + 1 == cs->maxItems) {
        cs->maxItems += 32;
        Coord *newCoords = realloc(cs->items, cs->maxItems * sizeof(Coord));
        if (newCoords == NULL) {
            perror("Could not expand CoordSet item array.");
            exit(EXIT_FAILURE);
        }
        cs->items = newCoords;
    }
    // Add the item to the list
    cs->items[cs->nItems++] = c;
}

void csDestroy(CoordSet *cs) {
    free(cs->items);
}

int processP1(const char *data) {
    int nHouses;
    CoordSet coordSet = csInit(32);
    Coord c = {};
    csAppend(&coordSet, c);
    for (size_t i = 0; i < strlen(data); ++i) {
        char instr = data[i];
        switch (instr) {
            case '^': c.y++; break;
            case 'v': c.y--; break;
            case '>': c.x++; break;
            case '<': c.x--; break;
        }
        csAppend(&coordSet, c);
    }
    nHouses = coordSet.nItems;
    csDestroy(&coordSet);
    return nHouses;
}

int processP2(const char *data) {
    int nHouses;
    CoordSet coordSet = csInit(32);
    Coord s = {};
    Coord r = {};
    csAppend(&coordSet, s);
    for (size_t i = 0; i < strlen(data); ++i) {
        char instr = data[i];
        int dx = 0;
        int dy = 0;
        switch (instr) {
            case '^': dy++; break;
            case 'v': dy--; break;
            case '>': dx++; break;
            case '<': dx--; break;
        }
        if (i % 2 == 0) {
            s.x += dx;
            s.y += dy;
            csAppend(&coordSet, s);
        } else {
            r.x += dx;
            r.y += dy;
            csAppend(&coordSet, r);
        }
    }
    nHouses = coordSet.nItems;
    csDestroy(&coordSet);
    return nHouses;
}

void readFile(const char *filename, char *buf);

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Incorrect number of arguments given (%d).\n", argc);
        fprintf(stderr, "Usage: day03 <input-file>\n");
        exit(EXIT_FAILURE);
    }
    // Tests for P1
    {
        char *t1 = ">";
        char *t2 = "^>v<";
        char *t3 = "^v^v^v";
        assert(processP1(t1) == 2);
        assert(processP1(t2) == 4);
        assert(processP1(t3) == 2);
        printf("All P1 test cases passed.\n");
    }
    // Tests for P2
    {
        char *t1 = "^v";
        char *t2 = "^>v<";
        char *t3 = "^v^v^v^v^v";
        assert(processP2(t1) == 3);
        assert(processP2(t2) == 3);
        assert(processP2(t3) == 11);
    }
    char *input = malloc(N_INPUT_CHARS);
    if (input == NULL) {
        perror("Could not allocate input buffer.\n");
        exit(EXIT_FAILURE);
    }
    readFile(argv[1], input);
    printf("D03P1: %d\n", processP1(input));
    printf("D03P2: %d\n", processP2(input));
    free(input);
}

void readFile(const char *filename, char *buf) {
    FILE *f = fopen(filename, "r");
    if (f == NULL) {
        fprintf(stderr, "Could not open input file '%s'.\n", filename);
        exit(EXIT_FAILURE);
    }
    size_t idx = 0;
    while (!feof(f)) {
        buf[idx++] = fgetc(f);
    }
    fclose(f);
}
