import { WorkerProxySpec } from "../../../../../../../../ui/vendor/getto-application/action/worker/message"

import { RequestResetTokenEvent } from "../../../request_token/event"

import { ConvertBoardResult } from "../../../../../../../../ui/vendor/getto-application/board/kernel/data"
import { RequestResetTokenFields } from "../../../request_token/data"

export type RequestResetTokenProxyMaterial = Readonly<{
    requestToken: RequestToken["method"]
}>
export type RequestResetTokenProxyMessage = RequestToken["message"]
export type RequestResetTokenProxyResponse = RequestToken["response"]

type RequestToken = WorkerProxySpec<
    "requestToken",
    Readonly<{ fields: ConvertBoardResult<RequestResetTokenFields> }>,
    RequestResetTokenEvent
>
