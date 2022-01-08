import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import { ChangePasswordFields, ChangePasswordRemoteError } from "./data"

export interface ChangePasswordRemote {
    (fields: ChangePasswordFields): Promise<ChangePasswordRemoteResult>
}
export type ChangePasswordRemoteResult = RemoteResult<true, ChangePasswordRemoteError>
