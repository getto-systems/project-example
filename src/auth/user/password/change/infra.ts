import { RemoteResult } from "../../../../common/util/remote/infra"

import { LoginId } from "../../login_id/kernel/data"
import { ChangePasswordFields, ChangePasswordRemoteError, OverwritePasswordFields } from "./data"

export interface ChangePasswordRemote {
    (fields: ChangePasswordFields): Promise<ChangePasswordRemoteResult>
}
export interface OverwritePasswordRemote {
    (
        user: Readonly<{ loginId: LoginId }>,
        fields: OverwritePasswordFields,
    ): Promise<ChangePasswordRemoteResult>
}

export type ChangePasswordRemoteResult = RemoteResult<true, ChangePasswordRemoteError>
