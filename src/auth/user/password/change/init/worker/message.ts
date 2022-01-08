import {
    WorkerProxyMessage,
    WorkerProxyResponse,
} from "../../../../../../../ui/vendor/getto-application/action/worker/message"

import { ChangePasswordFields } from "../../data"
import { ChangePasswordRemoteResult } from "../../infra"

export type ChangePasswordProxyMessage = ChangePasswordRemoteMessage
export type ChangePasswordRemoteMessage = WorkerProxyMessage<
    "change-password-remote",
    { fields: ChangePasswordFields }
>

export type ChangePasswordProxyResponse = WorkerProxyResponse<
    "change-password-remote",
    ChangePasswordRemoteResult
>
