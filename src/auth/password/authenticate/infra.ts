import { DelayTime } from "../../../z_details/_ui/config/infra"
import { RemoteResult } from "../../../z_details/_ui/remote/infra"

import { AuthTicket } from "../../ticket/kernel/data"
import { AuthenticatePasswordFields, AuthenticatePasswordRemoteError } from "./data"

export type AuthenticatePasswordInfra = Readonly<{
    authenticate: AuthenticatePasswordRemote
    config: Readonly<{
        takeLongtimeThreshold: DelayTime
    }>
}>

export interface AuthenticatePasswordRemote {
    (fields: AuthenticatePasswordFields): Promise<AuthenticatePasswordRemoteResult>
}
export type AuthenticatePasswordRemoteResult = RemoteResult<
    AuthTicket,
    AuthenticatePasswordRemoteError
>
