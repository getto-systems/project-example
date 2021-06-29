import { h, render } from "preact"

import { newFindNextVersionView } from "../../../../avail/version/_ui/action_find_next/init"

import { MoveToLatestVersionEntry } from "../../../../avail/version/_ui/action_find_next/x_preact/move_to_latest_version"
import { foregroundOutsideFeature } from "../../../../x_outside_feature/_ui/common"

render(
    h(MoveToLatestVersionEntry, newFindNextVersionView(foregroundOutsideFeature())),
    document.body,
)
