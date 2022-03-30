import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import { LoginId } from "../../login_id/input/data"
import { ChangePasswordFields, ChangePasswordRemoteError, OverridePasswordFields } from "./data"

export interface ChangePasswordRemote {
    (fields: ChangePasswordFields): Promise<ChangePasswordRemoteResult>
}
export interface OverridePasswordRemote {
    (
        user: Readonly<{ loginId: LoginId }>,
        fields: OverridePasswordFields,
    ): Promise<ChangePasswordRemoteResult>
}

export type ChangePasswordRemoteResult = RemoteResult<true, ChangePasswordRemoteError>
