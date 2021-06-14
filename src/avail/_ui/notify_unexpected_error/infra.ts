import { RemoteResult } from "../../../z_details/_ui/remote/infra"

import { NotifyUnexpectedErrorRemoteError } from "./data"

export type NotifyUnexpectedErrorInfra = Readonly<{
    notify: NotifyUnexpectedErrorRemote
}>

export interface NotifyUnexpectedErrorRemote {
    (err: unknown): Promise<NotifyUnexpectedErrorRemoteResult>
}
export type NotifyUnexpectedErrorRemoteResult = RemoteResult<true, NotifyUnexpectedErrorRemoteError>
