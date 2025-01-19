#manifest
self as ui.skin
"client_skin/game.cob" as ui.skin.game

#import
builtin.colors.tailwind as tw
ui_common.constants as const

#defs
$COLOR_LOADSCREEN = $tw::STONE_400
$COLOR_LOADBAR_GUTTER = $tw::ZINC_400
$COLOR_LOADBAR = $tw::ZINC_600
$COLOR_GAMEOVER = $tw::NEUTRAL_800
$COLOR_GAME_BG = $tw::SKY_800
$COLOR_GAME_CLICKER = $tw::LIME_600
$COLOR_GAME_SECONDARY_BUTTONS = $tw::AMBER_600
$COLOR_GAME_SECONDARY_BUTTONS_DISABLED = $tw::NEUTRAL_600
$COLOR_GAME_SECONDARY_BUTTONS_TEXT_DISABLED = #AAAAAA

#scenes
"loadscreen"
    GlobalZIndex($const::ZINDEX_LOADSCREEN)
    FlexNode{width:100vw height:100vh flex_direction:Column justify_main:Center justify_cross:Center}
    BackgroundColor($COLOR_LOADSCREEN)

    "text"
        FlexNode{margin:{bottom:30px}}
        TextLine{text:"Loading..." size:35}

    "gutter"
        FlexNode{width:20% height:30px flex_direction:Row justify_main:FlexStart justify_cross:Center}
        Splat<Border>(1px)
        BorderColor(#000000)
        BackgroundColor($COLOR_LOADBAR_GUTTER)

        "bar"
            FlexNode{height:100%}
            BackgroundColor($COLOR_LOADBAR)

"gameover"
    FlexNode{width:100vw height:100vh flex_direction:Column justify_main:Center justify_cross:Center}
    BackgroundColor($COLOR_GAMEOVER)

    "text"
        TextLine{text:"GAME OVER" size:45}
        TextLineColor(#FFFFFF)
