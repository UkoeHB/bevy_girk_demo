#import
ui.skin as ui

#defs


#scenes
"scene"
    "area1"
        // lobby view

    "area2"
        // lobby list


"lobby_view"
    "header"

    "content"
        // needs to be scroll area in case too many members; only show scrollbar if scrollable

    "footer"
        // leave button
        // start game button
        // - only lobby owner can start the game once min number of players is present
        // - maybe add on-hover that indicates why the button is disabled

"lobby_member" // entry for lobby view



"lobby_list"
    "header"
        "title"

        "refresh_button"
            // re-requests the current page


        "refreshing_text"
            // only shows while refreshing


    "footer"
        // <<
        // <
        // [currently shown range]/total
        // >
        // >>

"lobby_list_entry"
    // lobby id
    // num members / total members allowed
    // join button


"game_ack_popup"
    // "Lobby Ready" title
    // timer
    // "Start" button
    // "Cancel" button


"make_lobby_popup"
    // "Make Lobby" title
    // password: not yet supported
    // num players (use arrow keys)
    // num watchers (use arrow keys)
    // make lobby button
    // cancel button


"join_lobby_popup"
    // "Join Lobby" title
    // password: not yet supported
    // join as: player, watcher (use radio button)
    // "Join" button
    // "Cancel" button





