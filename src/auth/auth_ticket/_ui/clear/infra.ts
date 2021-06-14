import { RemoteTypes } from "../../../../z_details/_ui/remote/infra"
import { AuthzRepositoryPod } from "../kernel/infra"
import { AuthnRepositoryPod } from "../kernel/infra"

import { ClearAuthTicketRemoteError } from "./data"

export type ClearAuthTicketInfra = Readonly<{
    authn: AuthnRepositoryPod
    authz: AuthzRepositoryPod
    clear: ClearAuthTicketRemotePod
}>

type ClearRemoteTypes = RemoteTypes<
    { type: "always" }, // 引数は必要ないが、null は嫌なのでこうしておく
    true,
    true,
    ClearAuthTicketRemoteError
>
export type ClearAuthTicketRemotePod = ClearRemoteTypes["pod"]
