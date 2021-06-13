import { ConvertLocationResult } from "../../../../../ui/vendor/getto-application/location/data"

export type SignViewType =
    | "static-privacyPolicy"
    | "password-reset-requestToken"
    | "password-reset-checkStatus"
    | "password-reset"

export type SignViewDetecter = Detect<SignViewType>

interface Detect<T> {
    (): ConvertLocationResult<T>
}
