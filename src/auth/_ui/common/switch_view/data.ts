import { ConvertLocationResult } from "../../../../z_details/_ui/location/data"

export type SignViewType =
    | "static-privacyPolicy"
    | "password-reset-requestToken"
    | "password-reset-checkStatus"
    | "password-reset"

export type SignViewDetecter = Detect<SignViewType>

interface Detect<T> {
    (): ConvertLocationResult<T>
}
