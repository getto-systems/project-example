import { RemoteResult } from "../../../../z_details/_ui/remote/infra"
import { AuthzRepository } from "../kernel/infra"
import { AuthnRepository } from "../kernel/infra"

import { ClearAuthTicketRemoteError } from "./data"

export type ClearAuthTicketInfra = Readonly<{
    authn: AuthnRepository
    authz: AuthzRepository
    clear: ClearAuthTicketRemote
}>

export interface ClearAuthTicketRemote {
    (): Promise<RemoteResult<true, ClearAuthTicketRemoteError>>
}
