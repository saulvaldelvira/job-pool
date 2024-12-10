#define _GNU_SOURCE
#include "../target/include/bindings.h"
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
