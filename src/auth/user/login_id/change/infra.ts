import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import { AuthUserAccountBasket } from "../../account/kernel/data"
import { ChangeLoginIdRemoteError, OverrideLoginIdFields } from "./data"

export interface OverrideLoginIdRemote {
    (
        user: AuthUserAccountBasket,
        fields: OverrideLoginIdFields,
    ): Promise<ChangePasswordRemoteResult>
}

export type ChangePasswordRemoteResult = RemoteResult<true, ChangeLoginIdRemoteError>
