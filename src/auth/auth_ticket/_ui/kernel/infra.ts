import { RemoteTypes } from "../../../../z_details/_ui/remote/infra"
import { RepositoryPod } from "../../../../z_details/_ui/repository/infra"

import { AuthTicket, Authn, Authz, RenewAuthTicketRemoteError } from "./data"

export type AuthnRepositoryPod = RepositoryPod<Authn, AuthnRepositoryValue>
export type AuthnRepositoryValue = Readonly<{
    authAt: string
}>

export type AuthzRepositoryPod = RepositoryPod<Authz, AuthzRepositoryValue>
export type AuthzRepositoryValue = Readonly<{
    roles: string[]
}>

type RenewRemoteTypes = RemoteTypes<
    { type: "always" }, // 引数は必要ないが、null を使うのは嫌なのでこうしておく
    AuthTicket,
    AuthRemoteValue,
    RenewAuthTicketRemoteError
>
export type RenewAuthTicketRemotePod = RenewRemoteTypes["pod"]
export type RenewAuthTicketResult = RenewRemoteTypes["result"]
export type RenewAuthTicketSimulator = RenewRemoteTypes["simulator"]

export type AuthRemoteValue = Readonly<{
    roles: string[]
}>
