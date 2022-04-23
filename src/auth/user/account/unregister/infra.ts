import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import { LoginId } from "../../login_id/kernel/data"
import { UnregisterAuthUserAccountRemoteError } from "./data"

export interface UnregisterAuthUserAccountRemote {
    (user: Readonly<{ loginId: LoginId }>): Promise<UnregisterAuthUserAccountRemoteResult>
}

export type UnregisterAuthUserAccountRemoteResult = RemoteResult<
    true,
    UnregisterAuthUserAccountRemoteError
>
