import { RemoteResult } from "../../../../common/util/remote/infra"

import { LoginId } from "../../login_id/kernel/data"
import { ModifyAuthUserAccountRemoteError, ModifyAuthUserAccountFields } from "./data"

export interface ModifyAuthUserAccountRemote {
    (
        user: Readonly<{ loginId: LoginId }> & ModifyAuthUserAccountFields,
        fields: ModifyAuthUserAccountFields,
    ): Promise<ModifyAuthUserAccountRemoteResult>
}

export type ModifyAuthUserAccountRemoteResult = RemoteResult<true, ModifyAuthUserAccountRemoteError>
