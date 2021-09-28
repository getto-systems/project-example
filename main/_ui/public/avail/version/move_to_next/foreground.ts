import { newFindNextVersionView } from "../../../../../../src/avail/version/action_find_next/init/resource"

import { MoveToNextVersionEntry } from "../../../../../../src/avail/version/action_find_next/x_plain/move_to_next_version"
import { newForegroundOutsideFeature } from "../../../../../../src/x_outside_feature/_ui/common"

MoveToNextVersionEntry(newFindNextVersionView(newForegroundOutsideFeature()))
