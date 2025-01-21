#defs
+scroll = \
    ScrollBase
    ControlRoot

    "view"
        ScrollView
        FlexNode{height:100% clipping:ScrollY flex_direction:Column justify_main:FlexStart justify_cross:FlexStart}

        // TODO: remove this extra node in bevy 0.15.1
        "shim"
            ScrollShim
            FlexNode{flex_direction:Column justify_main:FlexStart justify_cross:FlexStart}

    // The vertical scrollbar is invisible if there is no scrollable content.
    // - Invisible but *not* removed from layout.
    "vertical"
        ControlMember // Control group with scroll base
        FlexNode{height:100% width:18px}
        Multi<Static<Visibility>>[
            {value:Hidden}
            {state:[Custom("VerticalScroll")] value:Inherited}
        ]

        "gutter"
            FlexNode{height:100% width:100% flex_direction:Column justify_cross:Center padding:{top:6px bottom:6px}}
            BackgroundColor(#000000)

            "bar"
                ControlRoot
                ScrollBar{axis:Y}
                FlexNode{flex_grow:1 width:12px}
                BrRadius(2px)
                BackgroundColor(#26AAAAAA)

                "handle"
                    ScrollHandle
                    ControlMember // Control group with scrollbar
                    AbsoluteNode{width:100% height:100px} // Need pretend height for radius to work.
                    BrRadius(2px)
                    Responsive<BackgroundColor>{idle:#80BBBBBB hover:#B9EEEEEE}
\
