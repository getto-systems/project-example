import { FindNextVersionEvent } from "./event"

import { ApplicationTargetPath } from "./data"
import { LocationTypes } from "../../../../ui/vendor/getto-application/location/infra"

export interface FindNextVersionPod {
    (detecter: FindNextVersionLocationDetecter): FindNextVersionMethod
}
export interface FindNextVersionMethod {
    <S>(post: Post<FindNextVersionEvent, S>): Promise<S>
}

type FindNextVersionLocationTypes = LocationTypes<ApplicationTargetPath>
export type FindNextVersionLocationDetecter = FindNextVersionLocationTypes["detecter"]
export type FindNextVersionLocationDetectMethod = FindNextVersionLocationTypes["method"]
export type FindNextVersionLocationInfo = FindNextVersionLocationTypes["info"]
export type FindNextVersionLocationKeys = Readonly<{ version: string }>

interface Post<E, S> {
    (event: E): S
}
