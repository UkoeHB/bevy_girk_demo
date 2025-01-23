#defs
//todo: improve results when resizing window
+popup_button = \
    ControlRoot
    FlexNode{justify_main:Center justify_cross:Center}
    Multi<Responsive<BackgroundColor>>[
        {idle:#FFFFFF hover:#BBBBBB press:#999999} {state:[Disabled] idle:#AAAAAA}
    ]

    "text"
        ControlMember
        FlexNode{margin:{top:5px bottom:5px left:7px right:7px}}
        TextLine
        Multi<Static<TextLineColor>>[
            {value:#000000} {state:[Disabled] value:#CC000000}
        ]
\
+popup = \
    FlexNode{width:100vw height:100vh justify_main:Center justify_cross:Center}
    FocusPolicy::Block
    Picking::Block
    BackgroundColor(#90444444)

    "window"
        FlexNode{width:80% height:80% flex_direction:Column justify_main:Center justify_cross:Center}
        BackgroundColor(#000000)
        Splat<Border>(1px)
        BorderColor(#FFFFFF)

        "title"
            FlexNode{
                border:{bottom:1px}
                flex_direction:Row justify_main:Center justify_cross:Center
            }
            BorderColor(#FFFFFF)

            "text"
                TextLine
                TextLineSize(33)
                TextLineColor(#FFFFFF)

        "content"
            FlexNode{width:100% flex_grow:1 flex_direction:Column justify_main:FlexStart justify_cross:FlexStart}

        "footer"
            FlexNode{width:100% flex_direction:Row justify_main:SpaceEvenly justify_cross:Center}
            Padding{bottom:15px top:15px}

            "cancel_button"
                +popup_button{}

            "accept_button"
                +popup_button{}
\
