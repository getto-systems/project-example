import { ConvertLocationResult } from "../../../../z_details/_ui/location/data"

import { MenuTargetPath } from "./data"

export interface LoadMenuDetecter {
    (): ConvertLocationResult<MenuTargetPath>
}
