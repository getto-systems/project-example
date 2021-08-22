import { LogoutCoreAction, LogoutCoreState } from "./action"

export type LogoutResource = Readonly<{
    logout: LogoutCoreAction
}>

export type LogoutResourceState = Readonly<{ state: LogoutCoreState }>
