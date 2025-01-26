#import
ui.user as ui

#commands
LoadImages[
    "logo.png"
]

#defs
+menu_button = \
    RadioButton
    ControlRoot
    FlexNode{width:150px}
    Interactive
    Multi<Responsive<BackgroundColor>>[
        {idle:#000000 hover:#444444}
        {state:[Selected] idle:#666666}
        {state:[Custom("InLobby")] idle:#006666 hover:#007777}
        {state:[Selected Custom("InLobby")] idle:#008888}
    ]

    "text"
        ControlMember
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
            FlexNode{margin:{top:10px}}
            TextLine{text:"DEMO"}
            TextLineColor(#000000)

    ""
        FlexNode{height:1px width:80% margin:{top:6.5px}}
        BackgroundColor(#000000)

    "options"
        FlexNode{
            flex_grow:1 margin:{top:15px bottom:15px}
            flex_direction:Column justify_main:FlexStart justify_cross:Center
        }
        RadioGroup

    ""
        FlexNode{height:1px width:80% margin:{bottom:7px}}
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
        Margin{left:4px right:4px}
        TextLine
        TextLineColor(#000000)

    "status_text"
        Margin{left:4px right:4px}
        TextLine
        TextLineColor(#000000)
