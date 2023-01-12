import { RemoteResult } from "../../../../../common/util/remote/infra"

import { RequestResetTokenFields, RequestResetTokenRemoteError } from "./data"

export interface RequestResetTokenRemote {
    (fields: RequestResetTokenFields): Promise<RequestResetTokenRemoteResult>
}
export type RequestResetTokenRemoteResult = RemoteResult<true, RequestResetTokenRemoteError>
