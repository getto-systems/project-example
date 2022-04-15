import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import { LoginId } from "../../login_id/kernel/data"
import { GrantedAuthRole } from "../../kernel/data"
import { ModifyAuthUserAccountRemoteError, ModifyAuthUserAccountFields } from "./data"

export interface ModifyAuthUserAccountRemote {
    (
        user: Readonly<{ loginId: LoginId; grantedRoles: readonly GrantedAuthRole[] }>,
        fields: ModifyAuthUserAccountFields,
    ): Promise<ModifyAuthUserAccountRemoteResult>
}

export type ModifyAuthUserAccountRemoteResult = RemoteResult<
    true,
    ModifyAuthUserAccountRemoteError
>
