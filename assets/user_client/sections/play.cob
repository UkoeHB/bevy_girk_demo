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
ui.user.widgets as widgets

#defs
//todo: improve results when resizing window
+button = \
    ControlRoot
    FlexNode{justify_main:Center justify_cross:Center}
    Multi<Responsive<BackgroundColor>>[
        {idle:#FFFFFF hover:#AAAAAA press:#888888} {state:[Disabled] idle:#777777}
    ]

    "text"
        ControlMember
        FlexNode{margin:{top:5px bottom:5px left:7px right:7px}}
        TextLine
        Multi<Static<TextLineColor>>[
            {value:#000000} {state:[Disabled] value:#AA333333}
        ]
\

#scenes
"play"
    FlexNode{width:100% height:100% flex_direction:Column justify_main:FlexStart justify_cross:FlexStart}
    BackgroundColor(#000000)


"lobby_display"
    FlexNode{width:100% height:100% flex_direction:Column justify_main:FlexStart justify_cross:FlexStart}

    "header"
        FlexNode{width:100% justify_main:Center justify_cross:Center}

        "title"
            TextLine{text:"Current Lobby"}
            TextLineColor(#FFFFFF)

        "lobby_info"
            "text"
                TextLine
                TextLineColor(#FFFFFF)

        "member_count"
            FlexNode{flex_direction:Row justify_main:SpaceEvenly}

            "players"
                "text"
                    TextLine
                    TextLineColor(#FFFFFF)

            "watchers"
                "text"
                    TextLine
                    TextLineColor(#FFFFFF)

    "content"
        FlexNode{width:100% flex_grow:1 flex_direction:Column justify_main:FlexStart justify_cross:Center}

        "member_list"
            +widgets::scroll{
                FlexNode{min_width:70% height:80% flex_direction:Column justify_main:FlexStart justify_cross:FlexStart}
                Splat<Border>(1px)
                BorderColor(#FFFFFF)

                "view"
                    "shim"
            }

    "footer"
        FlexNode{width:100% flex_direction:Row justify_main:SpaceEvenly justify_cross:Center}

        "leave"
            FlexNode{flex_direction:Column justify_main:Center justify_cross:Center}
            "info"
                "text"
                    TextLine{text:"Back to lobby list" size:10}
            "button"
                +button{
                    +widgets::request_indicator{}
                    "text"
                        TextLine{text:"Leave"}
                }

        "start_button"
            +button{
                +widgets::request_indicator{}
                "text"
                    TextLine{text:"Start"}
            }

"lobby_display_member"
    "text"
        TextLine
        TextLineColor(#FFFFFF)



"lobby_list"
    FlexNode{width:100% height:100% flex_direction:Column justify_main:FlexStart justify_cross:FlexStart}

    "header"
        FlexNode{width:100%}

        "title"
            FlexNode{width:100% flex_direction:Row justify_main:Center justify_cross:Center}

            "text"
                TextLine{text:"Lobby List"}
                TextLineColor(#FFFFFF)

    "content"
        FlexNode{
            min_width:70% height:70%
            flex_direction:Column justify_main:FlexStart justify_cross:FlexStart justify_self_cross:Center
        }

        "upper_control"
            FlexNode{width:100% flex_direction:Row justify_main:FlexEnd justify_cross:Center}

            "loading_text"
                Multi<Static<Visibility>>[
                    {value:Inherited}
                    {state:[Disabled] value:Hidden}
                ]
                TextLine{text:"Loading..." size:10}
                TextLineColor(#FFFFFF)

            "refresh_button"
                +button{
                    "text"
                        FlexNode{margin:{top:3px bottom:3px left:5px right:5px}}
                        TextLine{text:"Refresh" size:13}
                }

        "list"
            +widgets::scroll{
                FlexNode{min_width:100% height:100% flex_direction:Column justify_main:FlexStart justify_cross:FlexStart}
                Splat<Border>(1px)
                BorderColor(#FFFFFF)

                "view"
                    "shim"
            }

        "controls"
            FlexNode{width:100% flex_direction:Row justify_main:SpaceEvenly justify_cross:Center}

            "paginate_now_button"
                +button{
                    "text"
                        FlexNode{margin:{top:4px bottom:4px left:15px right:15px}}
                        TextLine{text:"<<" size:13}
                }
            "paginate_left_button"
                +button{
                    "text"
                        FlexNode{margin:{top:4px bottom:4px left:15px right:15px}}
                        TextLine{text:"<" size:13}
                }
            "page_stats"
                "text"
                    TextLine{size:13}
                    TextLineColor(#FFFFFF)
            "paginate_right_button"
                +button{
                    "text"
                        FlexNode{margin:{top:4px bottom:4px left:15px right:15px}}
                        TextLine{text:">" size:13}
                }
            "paginate_oldest_button"
                +button{
                    "text"
                        FlexNode{margin:{top:4px bottom:4px left:15px right:15px}}
                        TextLine{text:">>" size:13}
                }

        "make_lobby_button"
            SetJustifySelfCross(Center)
            +button{
                +widgets::request_indicator{}
                "text"
                    TextLine{text:"Make Lobby" size:25}
            }

"lobby_list_entry"
    FlexNode{flex_direction:Row justify_main:FlexStart justify_cross:Center}
    "text"
        TextLine
        TextLineColor(#FFFFFF)

    "join_button"
        +button{
            "text"
                TextLine{text:"Join" size:15}
        }


"make_lobby_popup"
    +widgets::popup{
        "window"
            "title"
                "text"
                    TextLine{text:"New Lobby"}

            "content"
                SetJustifyMain(SpaceEvenly)
                SetJustifyCross(Center)
                Splat<Border>(1px)
                BorderColor(#FFFFFF)

                "password"
                    FlexNode{flex_direction:Row}

                    "fieldname"
                        TextLine{text:"Password:"}
                        TextLineColor(#FFFFFF)
                        Margin{right:5px}
                    "inputfield"
                        TextLine{text:"not yet supported..."}
                        TextLineColor(#FFFFFF)

                "max_players"
                    FlexNode{flex_direction:Row}

                    "fieldname"
                        TextLine{text:"Max Players:"}
                        TextLineColor(#FFFFFF)
                        Margin{right:5px}
                    "value"
                        TextLine
                        TextLineColor(#FFFFFF)
                    "buttons"
                        FlexNode{flex_direction:Row justify_self_cross:Center}
                        Margin{left:7px}
                        "remove_player_button"
                            +widgets::popup_button{
                                FlexNode{width:25px height:25px justify_main:Center justify_cross:Center}
                                "text"
                                    TextLine{text:"-" size:20}
                            }
                        ""
                            FlexNode{width:8px}
                        "add_player_button"
                            +widgets::popup_button{
                                FlexNode{width:25px height:25px justify_main:Center justify_cross:Center}
                                "text"
                                    TextLine{text:"+" size:20}
                            }
                "join_as"
                    FlexNode{flex_direction:Row}

                    "fieldname"
                        TextLine{text:"Join As:"}
                        TextLineColor(#FFFFFF)
                        Margin{right:5px}
                    "value"
                        TextLine
                        TextLineColor(#FFFFFF)

                "connection_notice"
                    AbsoluteNode{width:100% top:auto bottom:0% justify_main:Center justify_cross:Center}
                    "text"
                        TextLine{size:20}
                        TextLineColor(#FFFFFF)

            "footer"
                "cancel_button"
                    "text"
                        TextLine{text:"Cancel"}
                "accept_button"
                    +widgets::request_indicator{}
                    "text"
                        TextLine{text:"Make"}
    }


"join_lobby_popup"
    +widgets::popup{
        "window"
            "title"
                "text"
                    TextLine{text:"Join Lobby"}

            "subtitle"
                "text"
                    TextLine
                    TextLineColor(#FFFFFF)

            "content"
                SetJustifyMain(SpaceEvenly)
                SetJustifyCross(Center)
                "password"
                    FlexNode{flex_direction:Row}
                    "fieldname"
                        TextLine{text:"Password:"}
                        TextLineColor(#FFFFFF)
                        Margin{right:5px}
                    "inputfield"
                        TextLine{text:"not yet supported..."}
                        TextLineColor(#FFFFFF)
                "join_as"
                    FlexNode{flex_direction:Row}
                    "fieldname"
                        TextLine{text:"Join As:"}
                        TextLineColor(#FFFFFF)
                        Margin{right:5px}
                    "value"
                        TextLine
                        TextLineColor(#FFFFFF)

            "footer"
                "cancel_button"
                    "text"
                        TextLine{text:"Cancel"}
                "accept_button"
                    +widgets::request_indicator{}
                    "text"
                        TextLine{text:"Join"}
    }



