import { h, render } from "preact"

import { newFindNextVersionView } from "../../../../../avail/version/find_next/init/resource"
import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"

import { MoveToLatestVersion } from "../../../../../avail/version/find_next/x_preact/move_to_latest_version"

render(h(MoveToLatestVersion, newFindNextVersionView(newForegroundOutsideFeature())), document.body)
