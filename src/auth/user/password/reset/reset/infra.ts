import { DelayTime } from "../../../../../z_lib/ui/config/infra"
import { RemoteResult } from "../../../../../z_lib/ui/remote/infra"

import { AuthProfile } from "../../../../ticket/kernel/data"
import { ResetPasswordFields, ResetPasswordRemoteError } from "./data"
import { ResetToken } from "../../input/data"
import { ConvertLocationResult } from "../../../../../z_lib/ui/location/data"

export interface ResetPasswordDetecter {
    (): ConvertLocationResult<ResetToken>
}

export type ResetPasswordInfra = Readonly<{
    resetRemote: ResetPasswordRemote
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
export type ResetPasswordRemoteResult = RemoteResult<AuthProfile, ResetPasswordRemoteError>
