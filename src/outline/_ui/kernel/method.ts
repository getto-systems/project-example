import { ConvertLocationResult } from "../../../../ui/vendor/getto-application/location/data"

import { MenuTargetPath } from "./data"

export type LoadMenuDetecter = Detect<MenuTargetPath>

interface Detect<T> {
    (): ConvertLocationResult<T>
}
