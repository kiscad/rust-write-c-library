#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#include "hello.h"

int main(void) {
    int ret_code = 0;
    char *s = NULL;

    ret_code = get_some_cstr(&s);
    if (0 == ret_code) {
        printf("create a string: %s\n", s);
        printf("return code: %d\n", cstr_free(&s));
        printf("s addr after freeing: %p (expecting nil)\n", s);
    } else {
        printf("get_some_str res_code = %d\n", ret_code);
        return 10;
    }

    User *user = NULL;
    char name[] = "Goodmand";
    int age = 32;
    ret_code = user_new_with_result(&user, name, age);
    if (0 == ret_code) {
        printf("User { name: %s, age: %d}\n", user_name_get(user), user_age_get(user));
        printf("return code of freeing: %d\n", user_free_with_result(&user));
        printf("Point user after free: %p (expecting nil)\n", user);
    } else {
        printf("Error code: %d\n", ret_code);
    }

    User *user2 = user_new(name, age);
    printf("User { name: %s, age: %d}\n", user_name_get(user2), user_age_get(user2));
    user_free(user2);
    user2 = NULL;

    return 0;
}
