import { RemoteResult } from "../../../z_lib/ui/remote/infra"

import { RemoteCommonError } from "../../../z_lib/ui/remote/data"

export interface NotifyUnexpectedErrorRemote {
    (err: unknown): Promise<NotifyUnexpectedErrorRemoteResult>
}
export type NotifyUnexpectedErrorRemoteResult = RemoteResult<true, RemoteCommonError>
