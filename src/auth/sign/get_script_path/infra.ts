import { ConvertLocationResult } from "../../../common/util/location/data"
import { LocationPathname } from "./data"

export type GetScriptPathConfig = Readonly<{
    secureServerURL: SecureServerURL
}>

export type SecureServerURL = string & { SecureServerURL: never }

export type GetScriptPathShell = Readonly<{
    detectLocationPathname: LocationPathnameDetecter
}>

export interface LocationPathnameDetecter {
    (): ConvertLocationResult<LocationPathname>
}
