import { ConvertLocationResult } from "../../../z_lib/ui/location/data"

export type SignViewType =
    | "static-privacyPolicy"
    | "password-reset-requestToken"
    | "password-reset"

export type SignViewDetecter = Detect<SignViewType>

interface Detect<T> {
    (): ConvertLocationResult<T>
}
