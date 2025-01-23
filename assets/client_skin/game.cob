#import
ui.skin as _

#defs
+button = \
    FlexNode{justify_main:Center justify_cross:Center}
    Splat<Border>(1px)
    BorderColor(#000000)

    "text"
        FlexNode{margin:{top:5px bottom:5px left:7px right:7px}}
        TextLine
\
+scoreboard_header_item = \
    GridNode{border:{bottom:1px}}
    BorderColor(#000000)

    "text"
        FlexNode{margin:{bottom:2px left:5px right:5px}}
\

#scenes
"game"
    FlexNode{width:100vw height:100vh flex_direction:Column justify_main:FlexStart}
    BackgroundColor($COLOR_GAME_BG)

    "header"
        FlexNode{width:100% height:25px flex_direction:Row justify_cross:FlexStart justify_main:FlexStart}

        "name_shim"
            AbsoluteNode{width:100% height:100% justify_main:Center justify_cross:Center}

            "name"
                TextLine{size:25}

        "shim"
            FlexNode{flex_grow:1}

        "fps"
            FlexNode{width:50px justify_self_cross:FlexEnd}

            "text"
                TextLine{size:15}

    "content"
        FlexNode{width:100% flex_grow:1 flex_direction:Row justify_main:FlexStart justify_cross:FlexStart}

        "button_area"
            // Center the button in the remaining space.
            FlexNode{height:100% flex_grow:1 justify_main:Center justify_cross:Center}

            "click_button"
                +button{
                    Responsive<BackgroundColor>{
                        idle:$COLOR_GAME_CLICKER hover:$COLOR_GAME_CLICKER_HOVER press:$COLOR_GAME_CLICKER_PRESS
                    }

                    "text"
                        TextLine{text:"CLICK ME" size:35}
                        TextLineColor(#FFFFFF)
                }

        // Overlay scoreboard above content
        "scoreboard_shim"
            AbsoluteNode

            "scoreboard"
                // Scoreboard in upper left area of content.
                GridNode{
                    margin:{top:10px}
                    grid_auto_rows:[Auto]
                    grid_template_columns:[(Count(3), Auto)]
                }

                ""
                    +scoreboard_header_item{
                        "text"
                            TextLine{text:"Rank"}
                    }

                ""
                    +scoreboard_header_item{
                        "text"
                            TextLine{text:"Player"}
                    }

                ""
                    +scoreboard_header_item{
                        "text"
                            TextLine{text:"Score"}
                    }

    "footer"
        FlexNode{width:100% flex_direction:Row justify_cross:FlexStart justify_main:FlexStart}

        "disconnect_button"
            +button{
                SetJustifySelfCross(Center)
                Margin{left:10px bottom:10px}
                Responsive<BackgroundColor>{
                    idle:#00000000 hover:#55888888 press:#77888888
                }

                "text"
                    TextLine{text:"Disconnect" size:20}
            }

"scoreboard_rank_item"
    GridNode

    "text"
        FlexNode{margin:{bottom:2px left:5px right:5px top:2px}}
        TextLine

"scoreboard_player_item"
    GridNode

    "text"
        FlexNode{margin:{bottom:2px left:5px right:5px top:2px}}
        TextLine

"scoreboard_score_item"
    GridNode

    "text"
        FlexNode{margin:{bottom:2px left:5px right:5px top:2px}}
        TextLine




