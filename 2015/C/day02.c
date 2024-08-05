#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define N_INPUT_CHARS 8118

typedef struct {
    int l;
    int w;
    int h;
} Dimensions;

Dimensions toDimensions(const char *data) {
    Dimensions d;
    sscanf(data, "%dx%dx%d", &d.l, &d.w, &d.h);
    return d;
}

int getMin(int s1, int s2, int s3) {
    if (s1 <= s2 && s1 <= s3) return s1;
    else if (s2 <= s1 && s2 <= s3) return s2;
    else return s3;
}

int getFeetOfRibbonP1(Dimensions d) {
    int ft;
    int s1 = d.l * d.w;
    int s2 = d.w * d.h;
    int s3 = d.h * d.l;
    ft = 2 * s1 + 2 * s2 + 2 * s3;
    return ft + getMin(s1, s2, s3);
}

int getFeetOfRibbonP2(Dimensions d) {
    int s1 = 2 * d.l + 2 * d.w;
    int s2 = 2 * d.w + 2 * d.h;
    int s3 = 2 * d.h + 2 * d.l;
    return getMin(s1, s2, s3) + (d.l * d.w * d.h);
}

long int processP1(const char *data) {
    char *toParse = malloc(strlen(data));
    if (toParse == NULL) {
        perror("Failed to allocate memory.");
        exit(EXIT_FAILURE);
    }
    strcpy(toParse, data);
    char *tok = strtok(toParse, "\n");
    long int totalFeet = 0;
    while (tok != NULL) {
        if (strlen(tok) != 1) {
            Dimensions d = toDimensions(tok);
            totalFeet += (long int) getFeetOfRibbonP1(d);
        }
        tok = strtok(NULL, "\n");
    }
    free(toParse);
    return totalFeet;
}

long int processP2(const char *data) {
    char *toParse = malloc(strlen(data));
    if (toParse == NULL) {
        perror("Failed to allocate memory.");
        exit(EXIT_FAILURE);
    }
    strcpy(toParse, data);
    char *tok = strtok(toParse, "\n");
    long int totalFeet = 0;
    while (tok != NULL) {
        if (strlen(tok) != 1) {
            Dimensions d = toDimensions(tok);
            totalFeet += (long int) getFeetOfRibbonP2(d);
        }
        tok = strtok(NULL, "\n");
    }
    free(toParse);
    return totalFeet;
}

void readFile(char *filename, char *buf);

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Incorrect number of arguments given (%d).\n", argc);
        fprintf(stderr, "Usage: day02 <input-file>\n");
        exit(EXIT_FAILURE);
    }
    // Test getting dimentions
    {
        char *testStr = "2x3x4";
        Dimensions d = toDimensions(testStr);
        assert(d.l == 2);
        assert(d.w == 3);
        assert(d.h == 4);
        assert(getFeetOfRibbonP1(d) == 58);
    }
    char *input = malloc(N_INPUT_CHARS);
    readFile(argv[1], input);
    printf("D02P1: %ld\n", processP1(input));
    printf("D02P2: %ld\n", processP2(input));
    free(input);
    return 0;
}

void readFile(char *filename, char *buf) {
    FILE *f = fopen(filename, "r");
    size_t idx = 0;
    while (!feof(f)) {
        buf[idx++] = fgetc(f);
    }
    fclose(f);
}
