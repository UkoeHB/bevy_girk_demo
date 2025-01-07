#manifest
self as ui.skin
"client_skin/game.cob" as ui.skin.game

#import
constants as const

#scenes
"loadscreen"
    GlobalZIndex($const::ZINDEX_LOADSCREEN)
    FlexNode{width:100vw height:100vh flex_direction:Column justify_main:Center justify_cross:Center}
    BackgroundColor($const::COLOR_LOADSCREEN)

    "text"
        FlexNode{margin:{bottom:30px}}
        TextLine{text:"Loading..." size:35}

    "gutter"
        FlexNode{width:20% height:30px flex_direction:Row justify_main:FlexStart justify_cross:Center}
        Splat<Border>(1px)
        BorderColor(#000000)
        BackgroundColor($const::COLOR_LOADBAR_GUTTER)

        "bar"
            FlexNode{height:100%}
            BackgroundColor($const::COLOR_LOADBAR)

"gameover"
    FlexNode{width:100vw height:100vh flex_direction:Column justify_main:Center justify_cross:Center}
    BackgroundColor($const::COLOR_GAMEOVER)

    "text"
        TextLine{text:"GAME OVER" size:45}
        TextLineColor(#FFFFFF)
