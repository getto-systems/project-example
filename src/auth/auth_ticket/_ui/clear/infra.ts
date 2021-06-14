import { RemoteResult } from "../../../../z_details/_ui/remote/data"
import { AuthzRepositoryPod } from "../kernel/infra"
import { AuthnRepositoryPod } from "../kernel/infra"

import { ClearAuthTicketRemoteError } from "./data"

export type ClearAuthTicketInfra = Readonly<{
    authn: AuthnRepositoryPod
    authz: AuthzRepositoryPod
    clear: ClearAuthTicketRemote
}>

export interface ClearAuthTicketRemote {
    (): Promise<RemoteResult<true, ClearAuthTicketRemoteError>>
}
