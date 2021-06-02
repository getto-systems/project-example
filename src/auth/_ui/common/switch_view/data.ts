import { LocationTypes } from "../../../../../ui/vendor/getto-application/location/infra"

export type SignViewType =
    | "static-privacyPolicy"
    | "password-reset-requestToken"
    | "password-reset-checkStatus"
    | "password-reset"

type SignViewLocationTypes = LocationTypes<SignViewType>
export type SignViewLocationDetecter = SignViewLocationTypes["detecter"]
export type SignViewLocationDetectMethod = SignViewLocationTypes["method"]
export type SignViewLocationInfo = SignViewLocationTypes["info"]
