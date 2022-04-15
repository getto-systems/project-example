import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import { ChangeLoginIdRemoteError, OverrideLoginIdFields } from "./data"
import { LoginId } from "../kernel/data"

export interface OverrideLoginIdRemote {
    (
        user: Readonly<{ loginId: LoginId }>,
        fields: OverrideLoginIdFields,
    ): Promise<ChangePasswordRemoteResult>
}

export type ChangePasswordRemoteResult = RemoteResult<true, ChangeLoginIdRemoteError>
