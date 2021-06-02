import { RemoteTypes } from "../../../../ui/vendor/getto-application/infra/remote/infra"

import { NotifyUnexpectedErrorRemoteError } from "./data"

export type NotifyUnexpectedErrorInfra = Readonly<{
    notify: NotifyUnexpectedErrorRemotePod
}>

type NotifyUnexpectedErrorRemoteTypes = RemoteTypes<
    unknown,
    true,
    true,
    NotifyUnexpectedErrorRemoteError
>
export type NotifyUnexpectedErrorRemotePod = NotifyUnexpectedErrorRemoteTypes["pod"]
export type NotifyUnexpectedErrorRemoteResult = NotifyUnexpectedErrorRemoteTypes["result"]
export type NotifyUnexpectedErrorSimulator = NotifyUnexpectedErrorRemoteTypes["simulator"]
