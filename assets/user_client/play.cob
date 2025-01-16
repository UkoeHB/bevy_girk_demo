/*
Networked buttons should indicate request status.
- Add indicators 'floating' as a tooltip to the upper-right of the button text.
- Add 'pending' spiny for "RequestPending"
- Add 'success' checkmark for "RequestSucceeded"
    - Fades out after 2s
- Add 'failed' x mark for "RequestFailed"
    - Fades out after 2s
*/

#import
ui.skin as ui

#defs


#scenes
"play_section"



"lobby_display"
    "header"
        // leave button (closes lobby display view and returns the user to the list view)
        // - "Back to lobby list" in small text above button
        // title: Current Lobby

    "content"
        // needs to be scroll area in case too many members; only show scrollbar if scrollable

    "footer"
        // start game button
        // - only lobby owner can start the game once min number of players is present
        // - maybe add on-hover that indicates why the button is disabled

"lobby_member" // entry for lobby view



"lobby_list"
    "header"
        "title"

        "refresh_button"
            // re-requests the current page


        "loading_text"
            // only shows while loading
            // "Loading..."


    "footer"
        // <<
        // <
        // [currently shown range]/total
        // >
        // >>
        "make_lobby_button"
            // triggers popup

"lobby_list_entry"
    // text
    // join button


"ack_popup"
    // "Start Game" title
    // timer
    // "Accept" button
    // "Reject" button
    // - buttons use 'disabled' state to show they can't be used any more


"make_lobby_popup"
    // "New Lobby" title
    // password: not yet supported
    // max players buttons (+ "add_player"/- "remove_player")
    // make lobby button
    // cancel button


"join_lobby_popup"
    // "Join Lobby" title
    // password: not yet supported
    // join as: player, watcher (use radio button)
    // "Join" button
    // "Cancel" button





