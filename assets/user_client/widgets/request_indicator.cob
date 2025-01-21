#import
builtin.colors.tailwind as tw

#defs
// Should be inserted to a button with ControlRoot.
+request_indicator = \
    "frame"
        ControlMember
        AbsoluteNode{width:100% flex_direction:Row}
        Multi<Animated<PropagateOpacity>>[
            {idle:0}
            {state:[Custom("RequestPending")] idle:1}
            {state:[Custom("RequestSucceeded")] enter_ref:1 idle:0 enter_idle_with:{delay:0.75 duration:0.2 ease:ExpOut}}
            {state:[Custom("RequestFailed")] enter_ref:1 idle:0 enter_idle_with:{delay:0.75 duration:0.2 ease:ExpOut}}
        ]

        "shim"
            FlexNode{flex_grow:1}

        "ref_point"
            FlexNode{width:0px height:0px flex_direction:ColumnReverse}

            "indicator"
                ControlMember
                AbsoluteNode{top:auto bottom:-2px left:2px right:auto}
                Multi<Static<TextLine>>[
                    {state:[Custom("RequestPending")] value:{text:"..." size:10}}
                    {state:[Custom("RequestSucceeded")] value:{text:"√" size:13}}
                    {state:[Custom("RequestFailed")] value:{text:"x" size:13}}
                ]
                Multi<Static<TextLineColor>>[
                    {state:[Custom("RequestPending")] value:$tw::BLUE_500}
                    {state:[Custom("RequestSucceeded")] value:$tw::GREEN_500}
                    {state:[Custom("RequestFailed")] value:$tw::RED_500}
                ]
\
