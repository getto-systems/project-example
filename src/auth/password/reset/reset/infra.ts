import { DelayTime } from "../../../../z_details/_ui/config/infra"
import { RemoteResult } from "../../../../z_details/_ui/remote/infra"

import { AuthTicket } from "../../../ticket/kernel/data"
import { ResetPasswordFields, ResetPasswordRemoteError } from "./data"
import { ResetToken } from "../../input/data"

export type ResetPasswordInfra = Readonly<{
    reset: ResetPasswordRemote
    config: Readonly<{
        takeLongtimeThreshold: DelayTime
    }>
}>

export type ResetPasswordMessage = Readonly<{
    resetToken: ResetToken
    fields: ResetPasswordFields
}>
export interface ResetPasswordRemote {
    (resetToken: ResetToken, fields: ResetPasswordFields): Promise<ResetPasswordRemoteResult>
}
export type ResetPasswordRemoteResult = RemoteResult<AuthTicket, ResetPasswordRemoteError>
