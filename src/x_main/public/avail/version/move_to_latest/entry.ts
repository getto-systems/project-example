import { h, render } from "preact"

import { newFindNextVersionView } from "../../../../../avail/version/find_next/init/resource"

import { MoveToLatestVersionEntry } from "../../../../../avail/version/find_next/x_preact/move_to_latest_version"
import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"

render(
    h(MoveToLatestVersionEntry, newFindNextVersionView(newForegroundOutsideFeature())),
    document.body,
)
