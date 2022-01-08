import { ConvertLocationResult } from "../../../z_lib/ui/location/data"
import { SignViewType } from "./data"

export interface SignViewTypeDetecter {
    (): ConvertLocationResult<SignViewType>
}
