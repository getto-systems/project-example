import {
    WorkerProxyMessage,
    WorkerProxyResponse,
} from "../../../../../../../common/util/worker/message"

import { RequestResetTokenFields } from "../../data"
import { RequestResetTokenRemoteResult } from "../../infra"

export type RequestResetTokenProxyMessage = RequestTokenRemoteMessage
export type RequestTokenRemoteMessage = WorkerProxyMessage<
    "request-token-remote",
    { fields: RequestResetTokenFields }
>

export type RequestResetTokenProxyResponse = WorkerProxyResponse<
    "request-token-remote",
    RequestResetTokenRemoteResult
>
