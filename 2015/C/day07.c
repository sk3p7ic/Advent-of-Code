#include <limits.h>
#include <stdlib.h>
#include <stdio.h>
#include <string.h>

void panic(const char* msg);
void expect(void* ptr, const char* msg);

void readFile(const char* filename, char* buf);


size_t n_vertices = 0;

typedef struct {
    char** items;
    size_t sz;
    size_t max_sz;
} FixedSet;

FixedSet initFixedSet(size_t fixed_size) {
    char** items = malloc(sizeof(char *) * fixed_size);
    expect(items, "Could not create items buffer for FixedSet.");
    FixedSet s = { items, 0, fixed_size };
    return s;
}

void addFixedSetItem(char* item, FixedSet* s) {
    if (s->sz + 1 > s->max_sz) {
        fprintf(stderr, "Ran out of room in FixedSet (max size %ld)\n", s->sz);
        panic("Cannot add item to FixedSet.");
    }
    for (size_t i = 0; i < s->sz; ++i) {
        // If the element is already in the FixedSet
        if (!strcmp(s->items[i], item)) {
            return;
        }
    }
    s->items[s->sz++] = item;
}

typedef struct {
    char* c1;
    char* c2;
    int len;
} Route;

Route parseRoute(const char* s) {
    Route r = { malloc(32), malloc(32), 0 };
    char c1[32];
    char c2[32];
    sscanf(s, "%s to %s = %d", c1, c2, &r.len);
    strncpy(r.c1, c1, strlen(c1));
    strncpy(r.c2, c2, strlen(c2));
    return r;
}

typedef struct {
    Route* routes;
    size_t nRoutes;
} RouteGroup;

RouteGroup getRoutes(const char* str) {
    Route* routes = malloc(sizeof(Route) * 32);
    expect(routes, "Could not allocate buffer to store Routes.");
    char* s = malloc(strlen(str));
    expect(s, "Could not allocate buffer to parse input to Routes.");
    strncpy(s, str, strlen(str));
    char* tok = malloc(64);
    expect(tok, "Could not allocate token buffer to parse lines to Routes.");
    tok = strtok(s, "\n");
    size_t nRoutes = 0;
    while (tok != NULL && strlen(tok) > 1) {
        routes[nRoutes++] = parseRoute(tok);
        tok = strtok(NULL, "\n");
    }
    RouteGroup rg = { routes, nRoutes };
    return rg;
}

typedef struct {
    int v[50];
} Hasher;

int hash(char* s) {
    // We'll ignore the case where strlen(s) < 2
    return s[0] + s[1] - ('A' + 'a');
}

Hasher initHasher(char** strs, size_t n_strs) {
    Hasher h = {};
    for (size_t i = 0; i < n_strs; ++i) {
        h.v[hash(strs[i])] = (int) i;
    }
    return h;
}

int hashPos(Hasher h, char* s) {
    return h.v[hash(s)];
}

/*
* Code borrowed / adapted from
* https://www.geeksforgeeks.org/prims-minimum-spanning-tree-mst-greedy-algo-5/
*/

size_t minKeyValueIdx(int keys[], int mst[]) {
    int min = INT_MAX;
    int min_idx = 0;

    for (size_t i = 0; i < n_vertices; ++i) {
        if (mst[i] == 0 && keys[i] < min) {
            min = keys[i];
            min_idx = i;
        }
    }
    return min_idx;
}

int primMST(int g[n_vertices][n_vertices]) {
    int parent[n_vertices];
    int keys[n_vertices];
    int mst[n_vertices];

    for (size_t i = 0; i < n_vertices; ++i) {
        keys[i] = INT_MAX;
        mst[i] = 0;
    }
    keys[0] = 0;
    parent[0] = -1;
    for (int c = 0; c < n_vertices - 1; ++c) {
        int u = minKeyValueIdx(keys, mst);
        mst[u] = 1;
        for (size_t v = 0; v < n_vertices; ++v) {
            if (g[u][v] && !mst[v] && g[u][v] < keys[v]) {
                parent[v] = u;
                keys[v] = g[u][v];
            }
        }
    }
    int distance = 0;
    printf("Path:\n");
    for (size_t i = 1; i < n_vertices; ++i) {
        printf("    %d - %ld    %d\n", parent[i], i, g[i][parent[i]]);
        distance += g[parent[i]][i];
    }
    return distance;
}

/*
* End borrowed / adapted code.
*/

int main(int argc, char *argv[]) {
    if (argc != 2) {
        fprintf(stderr, "Incorrect number of arguments given (%d).\n", argc);
        fprintf(stderr, "Usage: day07 <input-file>\n");
        exit(EXIT_FAILURE);
    }
    {
        char* routeStr = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141";
        RouteGroup routes = getRoutes(routeStr);
        FixedSet set = initFixedSet(routes.nRoutes + 1);
        for (size_t i = 0; i < routes.nRoutes; ++i) {
            addFixedSetItem(routes.routes[i].c1, &set);
            addFixedSetItem(routes.routes[i].c2, &set);
        }
        n_vertices = set.sz;
        Hasher hasher = initHasher(set.items, n_vertices);
        for (int i = 0; i < n_vertices; ++i) {
            printf("%s: %d\n", set.items[i], hashPos(hasher, set.items[i]));
        }
        int edges[n_vertices][n_vertices];
        for (int i = 0; i < n_vertices; ++i) {
            for (int j = 0; j < n_vertices; ++j) {
                edges[i][j] = 0;
            }
        }
        for (size_t i = 0; i < routes.nRoutes; ++i) {
            Route r = routes.routes[i];
            edges[hashPos(hasher, r.c1)][hashPos(hasher, r.c2)] = r.len;
        }
        for (int i = 0; i < n_vertices; ++i) {
            for (int j = 0; j < n_vertices; ++j) {
                printf(" %3d", edges[i][j]);
            }
            printf("\n");
        }
        int distance = primMST(edges);
        printf("P1 Test: %d\n", distance);
    }
    char* input = malloc(1024);
    expect(input, "Could not create buffer for reading input file.");
    readFile(argv[1], input);
    RouteGroup routes = getRoutes(input);
    FixedSet set = initFixedSet(routes.nRoutes + 1);
    for (size_t i = 0; i < routes.nRoutes; ++i) {
        addFixedSetItem(routes.routes[i].c1, &set);
        addFixedSetItem(routes.routes[i].c2, &set);
    }
    n_vertices = set.sz;
    Hasher hasher = initHasher(set.items, n_vertices);
    for (int i = 0; i < n_vertices; ++i) {
        printf("%s: %d\n", set.items[i], hashPos(hasher, set.items[i]));
    }
    int edges[n_vertices][n_vertices];
    for (int i = 0; i < n_vertices; ++i) {
        for (int j = 0; j < n_vertices; ++j) {
            edges[i][j] = 0;
        }
    }
    for (size_t i = 0; i < routes.nRoutes; ++i) {
        Route r = routes.routes[i];
        edges[hashPos(hasher, r.c1)][hashPos(hasher, r.c2)] = r.len;
    }
    for (int i = 0; i < n_vertices; ++i) {
        for (int j = 0; j < n_vertices; ++j) {
            printf(" %3d", edges[i][j]);
        }
        printf("\n");
    }
    printf("D07P1: %d\n", primMST(edges));
    return 0;
}

void panic(const char* msg) {
    fprintf(stderr, "%s\n", msg);
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
