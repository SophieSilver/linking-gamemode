#include "gamemode_client.h"

// add wrappers for functions here without `static inline`

/// wrapper for gamemode_request_start_for without inline attributes
int gamemode_request_start_for_wrapper(pid_t pid) {
    return gamemode_request_start_for(pid);
}