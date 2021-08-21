import { ConvertLocationResult } from "../../../../z_details/_ui/location/data"

import { MenuTargetPath } from "./data"

export type LoadMenuDetecter = Detect<MenuTargetPath>

interface Detect<T> {
    (): ConvertLocationResult<T>
}
