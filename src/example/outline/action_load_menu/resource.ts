import { LoadMenuAction, LoadMenuState } from "./action"

export type LoadMenuResource = Readonly<{
    menu: LoadMenuAction
}>
export type LoadMenuResourceState = Readonly<{
    state: LoadMenuState
}>
