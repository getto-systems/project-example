import { ApplicationView } from "../../../../ui/vendor/getto-application/action/action"

import { FindNextVersionCoreAction, FindNextVersionCoreState } from "./core/action"

export type FindNextVersionView = ApplicationView<FindNextVersionResource>

export type FindNextVersionResource = Readonly<{
    findNext: FindNextVersionCoreAction
}>
export type FindNextVersionResourceState = Readonly<{
    state: FindNextVersionCoreState
}>
