import { RemoteResult } from "../../../z_lib/ui/remote/infra"
import {
    FetchRepositoryResult,
    StoreRepositoryResult,
} from "../../../z_lib/ui/repository/infra"

import { AuthTicket, Authn, Authz, RenewAuthTicketRemoteError } from "./data"

export interface AuthnRepository {
    get(): Promise<FetchRepositoryResult<Authn>>
    set(value: Authn): Promise<StoreRepositoryResult>
    remove(): Promise<StoreRepositoryResult>
}
export type AuthnRepositoryValue = Readonly<{
    authAt: string
}>

export type AuthzRepositoryValue = Readonly<{
    roles: string[]
}>
export interface AuthzRepository {
    get(): Promise<FetchRepositoryResult<Authz>>
    set(value: Authz): Promise<StoreRepositoryResult>
    remove(): Promise<StoreRepositoryResult>
}

export interface RenewAuthTicketRemote {
    (): Promise<RemoteResult<AuthTicket, RenewAuthTicketRemoteError>>
}

export type AuthRemoteValue = Readonly<{
    roles: string[]
}>
