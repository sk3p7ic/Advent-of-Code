#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

bool containsMinVowels(const char *s, short min) {
    short nVowels = 0;
    for (size_t i = 0; i < strlen(s); ++i) {
        char c = s[i];
        if (c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u') {
            if (++nVowels == min) return true;
        }
    }
    return false;
}

bool hasDouble(const char *s) {
    for (size_t i = 0; i < strlen(s) - 1; ++i) {
        if (s[i] == s[i + 1]) return true;
    }
    return false;
}

bool containsStrs(const char *hs, const char *needles[2], size_t nNeedles) {
    for (size_t i = 0; i < strlen(hs) - 1; ++i) {
        for (size_t j = 0; j < nNeedles; ++j) {
            const char *n = needles[j];
            if (hs[i] == n[0] && hs[i + 1] == n[1]) return true;
        }
    }
    return false;
}

bool isNiceP1(const char *s) {
    const char *needles[4] = {"ab", "cd", "pq", "xy"};
    return (containsMinVowels(s, 3) && hasDouble(s) &&
            !containsStrs(s, needles, 4));
}

int processP1(const char *input) {
    char *toParse = malloc(sizeof(input));
    if (toParse == NULL) {
        perror("Could not allocate a parseable buffer.");
        exit(EXIT_FAILURE);
    }
    strcpy(toParse, input);
    int nGood = 0;
    size_t nIter = 0;
    char *tok = strtok(toParse, "\n");
    while (tok != NULL) {
        if (isNiceP1(tok)) nGood++;
        tok = strtok(NULL, "\n");
        nIter++;
    }
    free(toParse);
    return nGood;
}

int processP2(const char *filename) {
    size_t cmdSize = strlen(filename) + 64;
    char *cmd = malloc(cmdSize);
    if (cmd == NULL) {
        perror("Could not allocate a command buffer.");
        exit(EXIT_FAILURE);
    }
    char *matchDoubleChars = "/usr/bin/grep \"\\(..\\).*\\1\"";
    char *matchEntrappedChars = "/usr/bin/grep \"\\(.\\).\\1\"";
    sprintf(cmd,
        "/bin/cat %s | %s | %s | /usr/bin/wc -l",
        filename,
        matchDoubleChars,
        matchEntrappedChars
    );
    FILE *resFp = popen(cmd, "r");
    if (resFp == NULL) {
        perror("Error when running commands for part 2.");
        exit(EXIT_FAILURE);
    }
    char resStr[16];
    fgets(resStr, sizeof(resStr), resFp);
    return atoi(resStr);
}

void readFile(const char *filename, char *buf);

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Incorrect number of arguments given (%d).\n", argc);
        fprintf(stderr, "Usage: day03 <input-file>\n");
        exit(EXIT_FAILURE);
    }
    // Test P1
    {
        const char *t1 = "ugknbfddgicrmopn";
        const char *t2 = "jchzalrnumimnmhp";
        const char *t3 = "haegwjzuvuyypxyu";
        const char *t4 = "dvszwmarrgswjxmb";
        assert(isNiceP1(t1) == true);
        assert(isNiceP1(t2) == false);
    }
    char *input = malloc(1000 * 17);
    readFile(argv[1], input);
    printf("D05P1: %d\n", processP1(input));
    printf("D05P2: %d\n", processP2(argv[1]));
    free(input);
}

void readFile(const char *filename, char *buf) {
    FILE *f = fopen(filename, "r");
    if (f == NULL) {
        perror("Could not read input file.");
        exit(EXIT_FAILURE);
    }
    size_t idx = 0;
    while (!feof(f)) {
        buf[idx++] = fgetc(f);
    }
    fclose(f);
}
