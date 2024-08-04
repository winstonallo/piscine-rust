#include <stdlib.h>
#include <string.h>

typedef unsigned int t_id;

typedef struct {
    t_id        id;
    char const* name;
} t_user;

typedef struct {
    t_id    next_user_id;
    t_user* users;
    size_t  count;
    size_t  allocated;
} t_database;

typedef enum {
    ERR_SUCCESS,
    ERR_MEMORY,
    ERR_NO_MORE_IDS,
    ERR_UNKNOWN_ID,
} e_result;

void delete_database(t_database* database);

e_result create_database(t_database* database) {
    if (!database) {
        return ERR_MEMORY;
    }

    database->users = (t_user*)calloc(10, sizeof(t_user));
    if (!database->users) {
        return ERR_MEMORY;
    }

    database->allocated = 10;
    database->next_user_id = 1;

    return ERR_SUCCESS;
}

void delete_database(t_database* database) {
    if (database) {
        for (size_t i = 0; i < database->count; ++i) {
            free((char*)database->users[i].name);
        }
        free(database->users);
    }
}

e_result create_user(t_database* database, const char* name, t_id* result) {
    if (database->count == database->allocated) {
        return ERR_NO_MORE_IDS;
    }

    t_user* new_user = &database->users[database->count];
    new_user->id = database->next_user_id++;
    new_user->name = strdup(name);
    if (!new_user->name) {
        return ERR_MEMORY;
    }

    *result = new_user->id;
    database->count++;

    return ERR_SUCCESS;
}

e_result delete_user(t_database* database, t_id id) {
    for (size_t i = 0; i < database->count; ++i) {
        if (database->users[i].id == id) {
            free((char*)database->users[i].name);
            for (size_t j = i; j < database->count - 1; ++j) {
                database->users[j] = database->users[j + 1];
            }
            database->count--;
            return ERR_SUCCESS;
        }
    }
    return ERR_UNKNOWN_ID;
}

e_result get_user(const t_database* database, t_id id, const t_user** result) {
    for (size_t i = 0; i < database->count; ++i) {
        if (database->users[i].id == id) {
            *result = &database->users[i];
            return ERR_SUCCESS;
        }
    }
    return ERR_UNKNOWN_ID;
}
