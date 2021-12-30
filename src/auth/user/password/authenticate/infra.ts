import { DelayTime } from "../../../../z_lib/ui/config/infra"
import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import { AuthProfile } from "../../../ticket/kernel/data"
import { AuthenticatePasswordFields, AuthenticatePasswordRemoteError } from "./data"

export type AuthenticatePasswordInfra = Readonly<{
    authenticateRemote: AuthenticatePasswordRemote
    config: Readonly<{
        takeLongtimeThreshold: DelayTime
    }>
}>

export interface AuthenticatePasswordRemote {
    (fields: AuthenticatePasswordFields): Promise<AuthenticatePasswordRemoteResult>
}
export type AuthenticatePasswordRemoteResult = RemoteResult<
    AuthProfile,
    AuthenticatePasswordRemoteError
>
