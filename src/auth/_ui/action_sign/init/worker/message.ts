import {
    RequestPasswordResetTokenProxyMessage,
    RequestPasswordResetTokenProxyResponse,
} from "../../../../password/reset/_ui/action_request_token/init/worker/message"

export type ForegroundMessage = Readonly<{
    type: "password-reset-requestToken"
    message: RequestPasswordResetTokenProxyMessage
}>

export type BackgroundMessage =
    | Readonly<{
          type: "password-reset-requestToken"
          response: RequestPasswordResetTokenProxyResponse
      }>
    | Readonly<{ type: "error"; err: string }>
