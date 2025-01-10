#import
ui.user as ui

#defs
+menu_button = \
    //change bg color when InLobby state is added
    //todo: add InLobby state to play button when InLobby event broadcast, then remove when LeftLobby event broadcast

    "text"
        TextLine
        TextLineSize(...)
        TextLineColor(...)
\

#scenes
//todo: spawn menu OnEnter(LoadState::Done) and OnExit(ClientAppState::Game)
"menu"
    "side_panel"
        //right border

        "header"

            "logo"
                //todo: update logo to be transparent instead of white

            "text"

        "separator"
            //partial line

        "options"
            RadioGroup

        "footer"
            // shows server connection status (color changes on status) and host server client id
            // - react to ConnectionStatus resource mutations

    "content"

"menu_home"
    +menu_button{
        "text"
            TextLine{text:"Home"}
    }

"menu_play"
    +menu_button{
        "text"
            TextLine{text:"Play"}
    }

"menu_settings"
    +menu_button{
        "text"
            TextLine{text:"Settings"}
    }

// popup when reconnecting to an ongoing game: display in ClientAppState::Client when ClientStarter has a game
// - FocusPolicy::Block, Picking::Block
// - "Reconnecting..." -> animate the "..."
