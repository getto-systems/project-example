import { RemoteResult } from "../../../../z_details/_ui/remote/infra"
import { AuthzRepository } from "../kernel/infra"
import { AuthnRepository } from "../kernel/infra"

import { LogoutRemoteError } from "./data"

export type LogoutInfra = Readonly<{
    authn: AuthnRepository
    authz: AuthzRepository
    logout: LogoutRemote
}>

export interface LogoutRemote {
    (): Promise<RemoteResult<true, LogoutRemoteError>>
}
