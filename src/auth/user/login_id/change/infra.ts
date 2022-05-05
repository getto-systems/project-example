import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import { ChangeLoginIdRemoteError, OverwriteLoginIdFields } from "./data"
import { LoginId } from "../kernel/data"

export interface OverwriteLoginIdRemote {
    (
        user: Readonly<{ loginId: LoginId }>,
        fields: OverwriteLoginIdFields,
    ): Promise<ChangePasswordRemoteResult>
}

export type ChangePasswordRemoteResult = RemoteResult<true, ChangeLoginIdRemoteError>
