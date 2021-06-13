import { ConvertLocationResult } from "../../../../ui/vendor/getto-application/location/data"
import { LocationTypes } from "../../../../ui/vendor/getto-application/location/infra"

import { MenuTargetPath } from "./data"

type LoadMenuLocationTypes = LocationTypes<MenuTargetPath>
export type LoadMenuDetecter = Detect<MenuTargetPath>
export type LoadMenuLocationDetectMethod = LoadMenuLocationTypes["method"]
export type LoadMenuLocationInfo = LoadMenuLocationTypes["info"]
export type LoadMenuLocationKeys = Readonly<{ version: string }>

interface Detect<T> {
    (): ConvertLocationResult<T>
}