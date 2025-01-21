#import
ui.user as ui

#commands
LoadImages[
    "logo.png"
]

#defs
+menu_button = \
    RadioButton
    FlexNode{min_width:100%}
    Interactive
    Multi<Responsive<BackgroundColor>>[
        {idle:#000000 hover:#111111}
        {state:[Selected] idle:#444444}
        {state:[InLobby] idle:#003333 hover:#004444}
        {state:[Selected InLobby] idle:#008888}
    ]

    "text"
        FlexNode{margin:{top:5px bottom:5px left:7px right:7px}}
        TextLine
        TextLineSize(25)
        TextLineColor(#FFFFFF)
\

#scenes
"sidebar"
    FlexNode{height:100% flex_direction:Column justify_main:Center justify_cross:Center}
    Splat<Border>(1px)
    BackgroundColor(#FFFFFF)
    BorderColor(#000000)

    "header"
        FlexNode{flex_direction:Column justify_main:Center justify_cross:Center}

        "logo"
            LoadedImageNode{image:"logo.png"}

        "text"
            TextLine{text:"DEMO"}
            TextLineColor(#000000)

    ""
        FlexNode{height:1px width:80%}
        BackgroundColor(#000000)

    "options"
        FlexNode{flex_grow:1 flex_direction:Column justify_main:FlexStart justify_cross:Center}
        RadioGroup

    ""
        FlexNode{height:1px width:80%}
        BackgroundColor(#000000)

    "footer"
        FlexNode{flex_direction:Column justify_main:FlexStart justify_cross:Center}

"play_button"
    +menu_button{
        "text"
            Multi<Static<TextLine>>[
                {value:{text:"Play"}}
                {state:[Custom("InLobby")], value:{text:"In Lobby"}}
            ]
    }

"home_button"
    +menu_button{
        "text"
            TextLine{text:"Home"}
    }

"settings_button"
    +menu_button{
        "text"
            TextLine{text:"Settings"}
    }

"user_info"
    FlexNode{flex_direction:Column justify_main:FlexStart justify_cross:Center}

    "id_text"
        TextLine
        TextLineColor(#000000)

    "status_text"
        TextLine
        TextLineColor(#000000)
