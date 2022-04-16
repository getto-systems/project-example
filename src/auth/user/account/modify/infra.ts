import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import { LoginId } from "../../login_id/kernel/data"
import { AuthRole } from "../../kernel/data"
import { ModifyAuthUserAccountRemoteError, ModifyAuthUserAccountFields } from "./data"

export interface ModifyAuthUserAccountRemote {
    (
        user: Readonly<{ loginId: LoginId; grantedRoles: readonly AuthRole[] }>,
        fields: ModifyAuthUserAccountFields,
    ): Promise<ModifyAuthUserAccountRemoteResult>
}

export type ModifyAuthUserAccountRemoteResult = RemoteResult<
    true,
    ModifyAuthUserAccountRemoteError
>
