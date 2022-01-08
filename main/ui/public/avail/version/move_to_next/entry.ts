import { newFindNextVersionView } from "../../../../../../src/avail/version/find_next/init/resource"

import { MoveToNextVersionEntry } from "../../../../../../src/avail/version/find_next/x_plain/move_to_next_version"
import { newForegroundOutsideFeature } from "../../../../../../src/x_outside_feature/common"

MoveToNextVersionEntry(newFindNextVersionView(newForegroundOutsideFeature()))
