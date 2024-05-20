#include <stdio.h>
#include "meteo.h"

int main() {
    printf("Calling Rust function from C...\n");
    rust_function();
    return 0;
}
