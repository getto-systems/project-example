import { newFindNextVersionAction } from "../../../../../avail/version/find_next/detail/resource"

import { MoveToNextVersion } from "../../../../../avail/version/find_next/x_plain/move_to_next_version"
import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"

MoveToNextVersion({
    findNext: newFindNextVersionAction(newForegroundOutsideFeature()),
})
