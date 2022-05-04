import { RemoteResult } from "../../../../../z_lib/ui/remote/infra"

import { AuthTicket } from "../../../../ticket/kernel/data"
import { ResetToken, ResetPasswordFields, ResetPasswordRemoteError } from "./data"
import { ConvertLocationResult } from "../../../../../z_lib/ui/location/data"

export interface ResetTokenDetecter {
    (): ConvertLocationResult<ResetToken>
}

export interface ResetPasswordRemote {
    (resetToken: ResetToken, fields: ResetPasswordFields): Promise<ResetPasswordRemoteResult>
}
export type ResetPasswordRemoteResult = RemoteResult<AuthTicket, ResetPasswordRemoteError>
