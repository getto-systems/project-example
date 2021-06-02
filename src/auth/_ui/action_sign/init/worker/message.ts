import {
    CheckPasswordResetSendingStatusProxyMessage,
    CheckPasswordResetSendingStatusProxyResponse,
} from "../../../../password/reset/ui/action_check_status/init/worker/message"
import {
    RequestPasswordResetTokenProxyMessage,
    RequestPasswordResetTokenProxyResponse,
} from "../../../../password/reset/ui/action_request_token/init/worker/message"

export type ForegroundMessage =
    | Readonly<{
          type: "password-reset-requestToken"
          message: RequestPasswordResetTokenProxyMessage
      }>
    | Readonly<{
          type: "password-reset-checkStatus"
          message: CheckPasswordResetSendingStatusProxyMessage
      }>

export type BackgroundMessage =
    | Readonly<{
          type: "password-reset-requestToken"
          response: RequestPasswordResetTokenProxyResponse
      }>
    | Readonly<{
          type: "password-reset-checkStatus"
          response: CheckPasswordResetSendingStatusProxyResponse
      }>
    | Readonly<{ type: "error"; err: string }>
