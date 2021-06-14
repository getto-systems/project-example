import { DelayTime } from "../../../../z_details/_ui/config/infra"

import { AuthTicket } from "../../../auth_ticket/_ui/kernel/data"
import { AuthenticatePasswordFields, AuthenticatePasswordRemoteError } from "./data"
import { RemoteResult } from "../../../../z_details/_ui/remote/data"

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
