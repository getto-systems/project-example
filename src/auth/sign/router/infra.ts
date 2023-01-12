import { ConvertLocationResult } from "../../../common/util/location/data"
import { SignViewType } from "./data"

export interface SignViewTypeDetecter {
    (): ConvertLocationResult<SignViewType>
}
