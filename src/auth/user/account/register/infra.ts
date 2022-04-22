import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import { AuthUserAccount } from "../kernel/data"
import { RegisterAuthUserAccountRemoteError } from "./data"

export interface RegisterAuthUserAccountRemote {
    (fields: AuthUserAccount): Promise<RegisterAuthUserAccountRemoteResult>
}

export type RegisterAuthUserAccountRemoteResult = RemoteResult<
    true,
    RegisterAuthUserAccountRemoteError
>
