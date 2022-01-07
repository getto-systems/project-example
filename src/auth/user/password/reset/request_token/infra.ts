import { RemoteResult } from "../../../../../z_lib/ui/remote/infra"

import { RequestResetTokenFields, RequestResetTokenRemoteError } from "./data"

export interface RequestResetTokenRemote {
    (fields: RequestResetTokenFields): Promise<RequestResetTokenRemoteResult>
}
export type RequestResetTokenRemoteResult = RemoteResult<true, RequestResetTokenRemoteError>
