#include <stdio.h>

int main() {
    struct {
        char*str;
        int num;
    } sneed[] = { // anonymous structs ftw
        {"Fizz", 3},
        {"Buzz", 5},
    };
    int sneed_len = sizeof(sneed) / sizeof(sneed[0]);

    for (int i = 1; i <= 25; i += 1) {
        int sneeded = 0;
        for (int j = 0; j < sneed_len; j += 1) {
            if (i % sneed[j].num == 0) {
                printf("%s", sneed[j].str);
                sneeded = 1;
            }
        }

        if (!sneeded) {
            printf("%d", i);
        }
        printf("\n");
    }
}