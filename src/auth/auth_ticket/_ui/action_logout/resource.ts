import { LogoutCoreAction, LogoutCoreState } from "./core/action"

export type LogoutResource = Readonly<{
    logout: LogoutCoreAction
}>

export type LogoutResourceState = Readonly<{ state: LogoutCoreState }>
