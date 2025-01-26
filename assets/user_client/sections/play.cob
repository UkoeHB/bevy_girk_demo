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
ui_common.constants as const

#defs
$lobby_list_line_color = #77FFFFFF
$lobby_list_text_size = 20
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
        FlexNode{
            width:100% margin:{top:10px bottom:6px}
            flex_direction:Column justify_main:Center justify_cross:Center
        }

        "title"
            TextLine{text:"Current Lobby"}
            TextLineColor(#FFFFFF)

        "lobby_info"
            "text"
                TextLine{size:20}
                TextLineColor(#FFFFFF)

        "member_count"
            FlexNode{flex_direction:Row justify_main:SpaceEvenly}

            "players"
                "text"
                    TextLine{size:20}
                    TextLineColor(#FFFFFF)

            ""
                FlexNode{width:10px}

            "watchers"
                "text"
                    TextLine{size:20}
                    TextLineColor(#FFFFFF)

    "content"
        FlexNode{width:100% flex_grow:1 flex_direction:Column justify_main:FlexStart justify_cross:Center}

        "member_list"
            +widgets::scroll{
                FlexNode{
                    min_width:500px width:80% flex_grow:1
                    flex_direction:Row justify_main:FlexStart justify_cross:FlexStart
                }
                Splat<Border>(1px)
                BorderColor(#FFFFFF)

                "view"
                    "shim"
                        Padding{top:6.5px left:10px}
            }

    "footer"
        FlexNode{
            width:100% margin:{top:25px}
            flex_direction:Row justify_main:SpaceEvenly justify_cross:Center
        }

        "leave"
            FlexNode{flex_direction:Column justify_main:Center justify_cross:Center}
            "info"
                AbsoluteNode{top:-17px left:auto}
                "text"
                    TextLine{text:"Back to lobby list" size:12}
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
    Margin{bottom:7px}
    "text"
        TextLine
        TextLineColor(#FFFFFF)



"lobby_list"
    FlexNode{width:100% height:100% flex_direction:Column justify_main:FlexStart justify_cross:FlexStart}

    "header"
        FlexNode{width:100%}

        "title"
            FlexNode{
                width:100% margin:{top:20px bottom:0px}
                flex_direction:Row justify_main:Center justify_cross:Center}

            "text"
                TextLine{text:"Lobby List"}
                TextLineColor(#FFFFFF)

    "content"
        FlexNode{
            width:500px flex_grow:1
            flex_direction:Column justify_main:FlexStart justify_cross:FlexStart justify_self_cross:Center
        }

        "upper_control"
            FlexNode{
                width:100% margin:{bottom:4px}
                flex_direction:Row justify_main:FlexEnd justify_cross:Center
            }

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
                FlexNode{
                    width:100% flex_grow:1
                    flex_direction:Row justify_main:FlexStart justify_cross:FlexStart
                }
                Splat<Border>(1px)
                BorderColor(#FFFFFF)

                "view"
                    "shim"
                        AbsoluteNode{width:100% flex_direction:Column}
                        "header"
                            GridNode{
                                width:100%
                                grid_template_columns:[1.1fr 1.1fr 1fr 1fr 0.5fr]
                                grid_template_rows:[auto]
                            }

                            "lobby"
                                FlexNode{height:100% justify_main:Center justify_cross:Center}
                                Border{right:1px bottom:1px}
                                BorderColor($lobby_list_line_color)
                                ""
                                    Margin{top:5px bottom:5px}
                                    TextLine{text:"Lobby" size:$lobby_list_text_size}
                            "owner"
                                FlexNode{height:100% justify_main:Center justify_cross:Center}
                                Border{right:1px bottom:1px}
                                BorderColor($lobby_list_line_color)
                                ""
                                    Margin{top:5px bottom:5px}
                                    TextLine{text:"Owner" size:$lobby_list_text_size}
                            "players"
                                FlexNode{height:100% justify_main:Center justify_cross:Center}
                                Border{right:1px bottom:1px}
                                BorderColor($lobby_list_line_color)
                                ""
                                    Margin{top:5px bottom:5px}
                                    TextLine{text:"Players" size:$lobby_list_text_size}
                            "watchers"
                                FlexNode{height:100% justify_main:Center justify_cross:Center}
                                Border{right:1px bottom:1px}
                                BorderColor($lobby_list_line_color)
                                ""
                                    Margin{top:5px bottom:5px}
                                    TextLine{text:"Watchers" size:$lobby_list_text_size}
                            "join_shim"
                                FlexNode{height:100%}
                                Border{bottom:1px}
                                BorderColor($lobby_list_line_color)
                        "entries"
                            GridNode{
                                width:100%
                                grid_template_columns:[1.1fr 1.1fr 1fr 1fr 0.5fr]
                                grid_auto_rows:[30px]
                            }
            }

        "controls"
            FlexNode{width:100% flex_direction:Row justify_main:SpaceEvenly justify_cross:Center}

            "paginate_now_button"
                +button{
                    "text"
                        FlexNode{margin:{top:4px bottom:4px left:15px right:15px}}
                        TextLine{text:"<<" size:15}
                }
            "paginate_left_button"
                +button{
                    "text"
                        FlexNode{margin:{top:4px bottom:4px left:15px right:15px}}
                        TextLine{text:"<" size:15}
                }
            "page_stats"
                "text"
                    TextLine{size:15}
                    TextLineColor(#FFFFFF)
            "paginate_right_button"
                +button{
                    "text"
                        FlexNode{margin:{top:4px bottom:4px left:15px right:15px}}
                        TextLine{text:">" size:15}
                }
            "paginate_oldest_button"
                +button{
                    "text"
                        FlexNode{margin:{top:4px bottom:4px left:15px right:15px}}
                        TextLine{text:">>" size:15}
                }

        "make_lobby_button"
            +button{
                SetJustifySelfCross(Center)
                Margin{top:25px}
                +widgets::request_indicator{}
                "text"
                    TextLine{text:"Make Lobby" size:25}
            }

"lobby_list_entry_lobby"
    FlexNode{height:100% justify_main:Center justify_cross:Center}
    Border{right:1px bottom:1px}
    BorderColor($lobby_list_line_color)
    "text"
        TextLine{size:$lobby_list_text_size}
        TextLineColor(#FFFFFF)
"lobby_list_entry_owner"
    FlexNode{height:100% justify_main:Center justify_cross:Center}
    Border{right:1px bottom:1px}
    BorderColor($lobby_list_line_color)
    "text"
        TextLine{size:$lobby_list_text_size}
        TextLineColor(#FFFFFF)
"lobby_list_entry_players"
    FlexNode{height:100% justify_main:Center justify_cross:Center}
    Border{right:1px bottom:1px}
    BorderColor($lobby_list_line_color)
    "text"
        TextLine{size:$lobby_list_text_size}
        TextLineColor(#FFFFFF)
"lobby_list_entry_watchers"
    FlexNode{height:100% justify_main:Center justify_cross:Center}
    Border{right:1px bottom:1px}
    BorderColor($lobby_list_line_color)
    "text"
        TextLine{size:$lobby_list_text_size}
        TextLineColor(#FFFFFF)
"lobby_list_entry_join_button"
    +button{
        -FlexNode
        FlexNode{height:100% justify_main:Center justify_cross:Center}
        "text"
            TextLine{text:"Join" size:15}
    }


"make_lobby_popup"
    +widgets::popup{
        GlobalZIndex($const::ZINDEX_MAKE_LOBBY_POPUP)
        "window"
            "title"
                "text"
                    TextLine{text:"New Lobby"}

            "content"
                SetJustifyMain(Center)
                SetJustifyCross(Center)

                "grid"
                    GridNode{
                        grid_template_columns:[auto auto]
                        grid_auto_rows:[70px]
                    }

                    "password_name"
                        FlexNode{height:100% margin:{right:10px} flex_direction:Row justify_main:FlexEnd justify_cross:Center}
                        "text"
                            TextLine{text:"Password:"}
                            TextLineColor(#FFFFFF)
                            Margin{right:5px}
                    "password_field"
                        FlexNode{height:100% flex_direction:Row justify_main:FlexStart justify_cross:Center}
                        "text"
                            TextLine{text:"unsupported"}
                            TextLineColor(#FFFFFF)

                    "max_players_name"
                        FlexNode{height:100% margin:{right:10px} flex_direction:Row justify_main:FlexEnd justify_cross:Center}
                        "text"
                            TextLine{text:"Max Players:"}
                            TextLineColor(#FFFFFF)
                            Margin{right:5px}
                    "max_players_field"
                        FlexNode{height:100% flex_direction:Row justify_main:FlexStart justify_cross:Center}
                        "text"
                            FlexNode{width:25px}
                            TextLine
                            TextLineColor(#FFFFFF)
                        "buttons"
                            FlexNode{flex_direction:Row justify_self_cross:Center}
                            Margin{left:2px}
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
                    "join_as_name"
                        FlexNode{height:100% margin:{right:10px} flex_direction:Row justify_main:FlexEnd justify_cross:Center}
                        "text"
                            TextLine{text:"Join As:"}
                            TextLineColor(#FFFFFF)
                            Margin{right:5px}
                    "join_as_field"
                        FlexNode{height:100% flex_direction:Row justify_main:FlexStart justify_cross:Center}
                        "text"
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
        GlobalZIndex($const::ZINDEX_JOIN_LOBBY_POPUP)
        "window"
            "title"
                "text"
                    TextLine{text:"Join Lobby"}

            "subtitle"
                Margin{top:10px}
                "text"
                    TextLine{size:22}
                    TextLineColor(#FFFFFF)

            "content"
                SetJustifyMain(Center)
                SetJustifyCross(Center)
                "grid"
                    GridNode{
                        grid_template_columns:[auto auto]
                        grid_auto_rows:[70px]
                    }

                    "password_name"
                        FlexNode{height:100% margin:{right:10px} flex_direction:Row justify_main:FlexEnd justify_cross:Center}
                        "text"
                            TextLine{text:"Password:"}
                            TextLineColor(#FFFFFF)
                            Margin{right:5px}
                    "password_field"
                        FlexNode{height:100% flex_direction:Row justify_main:FlexStart justify_cross:Center}
                        "text"
                            TextLine{text:"unsupported"}
                            TextLineColor(#FFFFFF)

                    "join_as_name"
                        FlexNode{height:100% margin:{right:10px} flex_direction:Row justify_main:FlexEnd justify_cross:Center}
                        "text"
                            TextLine{text:"Join As:"}
                            TextLineColor(#FFFFFF)
                            Margin{right:5px}
                    "join_as_field"
                        FlexNode{height:100% flex_direction:Row justify_main:FlexStart justify_cross:Center}
                        "text"
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



