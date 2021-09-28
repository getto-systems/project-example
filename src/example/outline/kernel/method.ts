import { ConvertLocationResult } from "../../../z_lib/ui/location/data"

import { MenuTargetPath } from "./data"

export interface LoadMenuDetecter {
    (): ConvertLocationResult<MenuTargetPath>
}
