#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N_INPUT_CHARS 7001

typedef struct {
    char *input;
    int expected;
} TestCase;

int charToValue(char c) {
    int v;
    switch (c) {
        case '(': v = 1; break;
        case ')': v = -1; break;
        default: v = 0;
    }
    return v;
}

int processP1(const char *data) {
    int floor = 0;
    for (size_t i = 0; i < strlen(data); ++i) {
        floor += charToValue(data[i]);
    }
    return floor;
}

size_t processP2(const char *data) {
    int floor = 0;
    for (size_t i = 0; i < strlen(data); ++i) {
        if (floor == -1) return i;
        floor += charToValue(data[i]);
    }
    return (size_t) strlen(data);
}

void testP1() {
    TestCase tests[] = {
        { "(())", 0 }, { "()()", 0 },
        { "(((", 3 }, { "(()(()(", 3 },
        { "())", -1 }, { "))(", -1 }
    };
    for (size_t i = 0; i < 6; ++i) {
        TestCase t = tests[i];
        printf("TEST: %s => End %d\n", t.input, t.expected);
        assert(processP1(t.input) == t.expected);
        printf("    |> true\n");
    }
}

void testP2() {
    TestCase tests[] = { { ")", 1 }, { "()())", 5 } };
    for (size_t i = 0; i < 2; ++i) {
        TestCase t = tests[i];
        printf("TEST: %s => Idx %d\n", t.input, t.expected);
        assert(processP2(t.input) == t.expected);
        printf("    |> true\n");
    }
}

void readFile(char *filename, char *buf);

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Incorrect number of arguments given (%d).\n", argc);
        fprintf(stderr, "Usage: day01 <input-file>\n");
        return 1;
    }
    assert(charToValue('(') == 1);
    assert(charToValue(')') == -1);
    assert(charToValue('n') == 0);
    testP1();
    testP2();
    char *input = malloc(N_INPUT_CHARS);
    readFile(argv[1], input);
    printf("D0P1: %d\n", processP1(input));
    printf("D0P2: %d\n", (int) processP2(input));
    free(input);
    return 0;
}

void readFile(char *filename, char *buf) {
    FILE *f = fopen(filename, "r");
    fgets(buf, N_INPUT_CHARS, f);
    fclose(f);
}
