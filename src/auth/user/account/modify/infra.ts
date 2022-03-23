import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import { AuthUserAccountBasket } from "../kernel/data"
import { ModifyAuthUserAccountRemoteError, ModifyAuthUserAccountFields } from "./data"

export interface ModifyAuthUserAccountRemote {
    (
        user: AuthUserAccountBasket,
        fields: ModifyAuthUserAccountFields,
    ): Promise<ModifyAuthUserAccountRemoteResult>
}

export type ModifyAuthUserAccountRemoteResult = RemoteResult<
    AuthUserAccountBasket,
    ModifyAuthUserAccountRemoteError
>
