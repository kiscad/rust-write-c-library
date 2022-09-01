#include <stdint.h>
#include <stdio.h>
#include "revseq.h"

int main(void) {
    int64_t seq_lens[] = {7, 2, 3, 5};
    reverse_sequence(seq_lens, 4);
    return 0;
}