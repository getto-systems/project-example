import { RemoteResult } from "../../../../../../common/util/remote/infra"

import { LoginId } from "../../../../login_id/kernel/data"
import { ResetTokenDestination } from "../kernel/data"
import { ChangeResetTokenDestinationRemoteError } from "./data"

export interface ChangeResetTokenDestinationRemote {
    (
        user: Readonly<{ loginId: LoginId; resetTokenDestination: ResetTokenDestination }>,
        fields: ResetTokenDestination,
    ): Promise<ChangeResetTokenDestinationRemoteResult>
}

export type ChangeResetTokenDestinationRemoteResult = RemoteResult<
    true,
    ChangeResetTokenDestinationRemoteError
>
