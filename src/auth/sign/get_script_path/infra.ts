import { ConvertLocationResult } from "../../../z_lib/ui/location/data"
import { LocationPathname } from "./data"

export type GetScriptPathInfra = Readonly<{
    config: Readonly<{
        secureServerURL: SecureServerURL
    }>
}>

export type SecureServerURL = string & { SecureServerURL: never }

export interface GetScriptPathDetecter {
    (): ConvertLocationResult<LocationPathname>
}
