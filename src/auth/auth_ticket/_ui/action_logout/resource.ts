import { LogoutAction, LogoutState } from "./action"

export type LogoutResource = Readonly<{
    logout: LogoutAction
}>

export type LogoutResourceState = Readonly<{ state: LogoutState }>
