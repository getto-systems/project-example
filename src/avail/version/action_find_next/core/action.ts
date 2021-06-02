import { ApplicationStateAction } from "../../../../../ui/vendor/getto-application/action/action"

import { FindNextVersionMethod } from "../../find_next/method"

import { FindNextVersionEvent } from "../../find_next/event"

export type FindNextVersionCoreAction = ApplicationStateAction<FindNextVersionCoreState>

export type FindNextVersionMaterial = Readonly<{
    find: FindNextVersionMethod
}>

export type FindNextVersionCoreState =
    | Readonly<{ type: "initial-next-version" }>
    | FindNextVersionEvent

export const initialFindNextVersionCoreState: FindNextVersionCoreState = {
    type: "initial-next-version",
}
