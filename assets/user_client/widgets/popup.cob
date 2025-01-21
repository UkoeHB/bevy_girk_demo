#defs
//todo: improve results when resizing window
+popup_button = \
    ControlRoot
    FlexNode{justify_main:Center justify_cross:Center}
    Multi<Responsive<BackgroundColor>>[
        {idle:#FFFFFF hover:#EEEEEE press:#DDDDDD} {state:[Disabled] idle:#AAAAAA}
    ]

    "text"
        ControlMember
        FlexNode{margin:{top:5px bottom:5px left:7px right:7px}}
        TextLine
        Multi<Static<TextLineColor>>[
            {value:#000000} {state:[Disabled] value:#AA333333}
        ]
\
+popup = \
    FlexNode{width:80% height:80% flex_direction:Column}
    FocusPolicy::Block
    Picking::Block
    BackgroundColor(#000000)
    Splat<Border>(1px)
    BorderColor(#FFFFFF)

    "title"
        FlexNode{width:100% flex_direction:Row justify_main:Center justify_cross:Center}

        "text"
            TextLine
            TextLineSize(33)
            TextLineColor(#FFFFFF)

    "content"
        FlexNode{width:100% flex_grow:1 flex_direction:Column justify_main:FlexStart justify_cross:FlexStart}

    "footer"
        FlexNode{width:100% flex_direction:Row justify_main:SpaceEvenly justify_cross:Center}

        "cancel_button"
            +popup_button{}

        "accept_button"
            +popup_button{}
\
