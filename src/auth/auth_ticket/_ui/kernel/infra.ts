import { RemoteResult } from "../../../../z_details/_ui/remote/infra"
import {
    FetchRepositoryResult,
    RepositoryPod,
    StoreRepositoryResult,
} from "../../../../z_details/_ui/repository/infra"

import { AuthTicket, Authn, Authz, RenewAuthTicketRemoteError } from "./data"

export type AuthnRepositoryPod = RepositoryPod<Authn, AuthnRepositoryValue>
export interface AuthnRepository {
    get(): Promise<FetchRepositoryResult<Authn>>
    set(value: Authn): Promise<StoreRepositoryResult>
    remove(): Promise<StoreRepositoryResult>
}
export type AuthnRepositoryValue = Readonly<{
    authAt: string
}>

export type AuthzRepositoryPod = RepositoryPod<Authz, AuthzRepositoryValue>
export type AuthzRepositoryValue = Readonly<{
    roles: string[]
}>

export interface RenewAuthTicketRemote {
    (): Promise<RemoteResult<AuthTicket, RenewAuthTicketRemoteError>>
}

export type AuthRemoteValue = Readonly<{
    roles: string[]
}>
