import { ApplicationView } from "../../../../../ui/vendor/getto-application/action/action"

import { FindNextVersionAction, FindNextVersionState } from "./action"

export type FindNextVersionView = ApplicationView<FindNextVersionAction>

export type FindNextVersionResource = Readonly<{
    findNext: FindNextVersionAction
}>
export type FindNextVersionResourceState = Readonly<{
    state: FindNextVersionState
}>
