#include <stdio.h>
#include <conio.h>

int press() {
    // use non-english lang on windows will cause incorrect decode
   // printf("press any key to exit...\n");
    getch();
    return 0;
}