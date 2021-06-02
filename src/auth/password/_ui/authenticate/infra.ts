import { RemoteTypes } from "../../../../../ui/vendor/getto-application/infra/remote/infra"
import { DelayTime } from "../../../../../ui/vendor/getto-application/infra/config/infra"
import { Clock } from "../../../../../ui/vendor/getto-application/infra/clock/infra"

import { AuthRemoteValue } from "../../../auth_ticket/_ui/kernel/infra"

import { AuthTicket } from "../../../auth_ticket/_ui/kernel/data"
import { AuthenticatePasswordFields, AuthenticatePasswordRemoteError } from "./data"

export type AuthenticatePasswordInfra = Readonly<{
    authenticate: AuthenticatePasswordRemotePod
    clock: Clock
    config: Readonly<{
        takeLongtimeThreshold: DelayTime
    }>
}>

type AuthenticatePasswordRemoteTypes = RemoteTypes<
    AuthenticatePasswordFields,
    AuthTicket,
    AuthRemoteValue,
    AuthenticatePasswordRemoteError
>
export type AuthenticatePasswordRemotePod = AuthenticatePasswordRemoteTypes["pod"]
export type AuthenticatePasswordResult = AuthenticatePasswordRemoteTypes["result"]
export type AuthenticatePasswordSimulator = AuthenticatePasswordRemoteTypes["simulator"]
