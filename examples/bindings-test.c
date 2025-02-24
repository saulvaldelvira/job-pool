/*
 * Job pool C bindings example.
 *
 * Compile with:
 *  $ gcc -ljob_pool -L ../target/debug bindings-test.c -o bindings-test
 *
 * Run with:
 *  $ LD_LIBRARY_PATH=../target/debug ./bindings-test
 */

#define _GNU_SOURCE
#include "../target/include/job-pool.h"
#include <unistd.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>

#define rnd(min,max) (rand() % (max + 1 - min) + min)

void test(void) {
        sleep(rnd(1,5));
        printf("Hello from %d\n", gettid());
}

int main(void) {
        PoolConfig conf = pool_default_conf();
        conf.n_workers = 1000;

        ThreadPool *pool = pool_init(conf);

        for (int i = 0; i < 1000; i++) {
                pool_execute_job(pool, test);
        }

        pool_join(pool);
        pool_free(pool);
}
