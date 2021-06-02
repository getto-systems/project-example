import { LocationTypes } from "../../../../../../ui/vendor/getto-application/location/infra"
import { ConvertScriptPathResult, LocationPathname } from "./data"

export interface GetScriptPathPod {
    (info: GetScriptPathLocationDetecter): GetScriptPathMethod
}

type GetScriptPathLocationTypes = LocationTypes<LocationPathname>
export type GetScriptPathLocationDetecter = GetScriptPathLocationTypes["detecter"]
export type GetScriptPathLocationMethod = GetScriptPathLocationTypes["method"]
export type GetScriptPathLocationInfo = GetScriptPathLocationTypes["info"]

export interface GetScriptPathMethod {
    (): ConvertScriptPathResult
}
