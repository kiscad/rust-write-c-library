#ifndef LIB_HELLO
#define LIB_HELLO

#include <stdint.h>


int32_t get_some_cstr(char  **s);
int32_t cstr_free(char **s);

typedef struct _custom_struct User;

int32_t user_new_with_result(User **user, char *name, int32_t age);
int32_t user_free_with_result(User **user);
User* user_new(char *name, int32_t age);
void user_free(User *user);

const char* user_name_get(const User *user);
int user_age_get(const User *user);

#endif
