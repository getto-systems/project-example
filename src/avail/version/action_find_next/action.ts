import { ApplicationStateAction } from "../../../../ui/vendor/getto-application/action/action"

import { FindNextVersionMethod } from "../find_next/method"

import { FindNextVersionEvent } from "../find_next/event"

export type FindNextVersionAction = ApplicationStateAction<FindNextVersionState>

export type FindNextVersionMaterial = Readonly<{
    find: FindNextVersionMethod
}>

export type FindNextVersionState =
    | Readonly<{ type: "initial-next-version" }>
    | FindNextVersionEvent

export const initialFindNextVersionState: FindNextVersionState = {
    type: "initial-next-version",
}
