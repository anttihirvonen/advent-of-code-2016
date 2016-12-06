#define MAX(x, y) (((x) > (y)) ? (x) : (y))
#define MIN(x, y) (((x) < (y)) ? (x) : (y))

#include <stdio.h>

typedef struct { int e[3]; } triangle;

void swap(int* a, int* b)
{
    int temp;
    temp = *a;
    *a = *b;
    *b = temp;
}

void sort_edges(triangle* t)
{
    if (t->e[0] > t->e[1]) swap(t->e+0, t->e+1);
    if (t->e[1] > t->e[2]) swap(t->e+1, t->e+2);
    if (t->e[0] > t->e[1]) swap(t->e+1, t->e+0);
}

int is_valid(triangle t)
{
    sort_edges(&t);
    return (t.e[0] + t.e[1]) > t.e[2];
}

int main()
{
    FILE* file = fopen("input-03.txt", "r");

    triangle t[3];
    int valid_count = 0;

    while(!feof(file))
    {
        for (int i = 0; i < 3; ++i)
            fscanf(file, "%d %d %d", t[0].e+i, t[1].e+i, t[2].e+i);
        for (int i = 0; i < 3; ++i)
            valid_count += is_valid(t[i]);
    }

    printf("%d\n", valid_count);
}
