import { RemoteResult } from "../../../common/util/remote/infra"

import { RemoteCommonError } from "../../../common/util/remote/data"

export interface NotifyUnexpectedErrorRemote {
    (err: unknown): Promise<NotifyUnexpectedErrorRemoteResult>
}
export type NotifyUnexpectedErrorRemoteResult = RemoteResult<true, RemoteCommonError>
