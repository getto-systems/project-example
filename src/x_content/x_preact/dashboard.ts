import { h, VNode } from "preact"

import { box_double, container } from "../../z_vendor/getto-css/preact/design/box"

import { DisplaySeasonField } from "../../core/season/load/x_preact/display_season_field"

import { LoadSeasonAction } from "../../core/season/load/action"

type Props = Readonly<{
    season: LoadSeasonAction
}>
export function Dashboard(props: Props): VNode {
    return container([
        box_double({
            title: "GETTO Example",
            body: h(DisplaySeasonField, props),
        }),
    ])
}
