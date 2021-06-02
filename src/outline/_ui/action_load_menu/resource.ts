import { LoadMenuCoreAction, LoadMenuCoreState } from "./core/action"

export type LoadMenuResource = Readonly<{
    menu: LoadMenuCoreAction
}>
export type LoadMenuResourceState = Readonly<{
    state: LoadMenuCoreState
}>
