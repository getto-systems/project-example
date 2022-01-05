import { RemoteResult } from "../../../../z_lib/ui/remote/infra"

import { AuthTicket } from "../../../ticket/kernel/data"
import { AuthenticatePasswordFields, AuthenticatePasswordRemoteError } from "./data"

export interface AuthenticatePasswordRemote {
    (fields: AuthenticatePasswordFields): Promise<AuthenticatePasswordRemoteResult>
}
export type AuthenticatePasswordRemoteResult = RemoteResult<
    AuthTicket,
    AuthenticatePasswordRemoteError
>
