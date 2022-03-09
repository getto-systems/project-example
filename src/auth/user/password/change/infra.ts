import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import { AuthUserAccountBasket } from "../../account/kernel/data"
import { ChangePasswordFields, ChangePasswordRemoteError, OverridePasswordFields } from "./data"

export interface ChangePasswordRemote {
    (fields: ChangePasswordFields): Promise<ChangePasswordRemoteResult>
}
export interface OverridePasswordRemote {
    (
        user: AuthUserAccountBasket,
        fields: OverridePasswordFields,
    ): Promise<ChangePasswordRemoteResult>
}

export type ChangePasswordRemoteResult = RemoteResult<true, ChangePasswordRemoteError>
