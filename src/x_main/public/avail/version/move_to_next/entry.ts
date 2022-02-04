import { newFindNextVersionView } from "../../../../../avail/version/find_next/init/resource"

import { MoveToNextVersionEntry } from "../../../../../avail/version/find_next/x_plain/move_to_next_version"
import { newForegroundOutsideFeature } from "../../../../../x_outside_feature/common"

MoveToNextVersionEntry(newFindNextVersionView(newForegroundOutsideFeature()))
