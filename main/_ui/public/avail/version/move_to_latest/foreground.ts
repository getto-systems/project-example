import { h, render } from "preact"

import { newFindNextVersionView } from "../../../../../../src/avail/version/_ui/action_find_next/init/resource"

import { MoveToLatestVersionEntry } from "../../../../../../src/avail/version/_ui/action_find_next/x_preact/move_to_latest_version"
import { newForegroundOutsideFeature } from "../../../../../../src/x_outside_feature/_ui/common"

render(
    h(MoveToLatestVersionEntry, newFindNextVersionView(newForegroundOutsideFeature())),
    document.body,
)
