import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import { LoginId } from "../../login_id/input/data"
import { GrantedAuthRole } from "../input/data"
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
