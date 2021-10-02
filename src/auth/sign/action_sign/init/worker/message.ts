import {
    RequestResetTokenProxyMessage,
    RequestResetTokenProxyResponse,
} from "../../../../user/password/reset/action_request_token/init/worker/message"

export type SignForegroundMessage = Readonly<{
    type: "password-reset-requestToken"
    message: RequestResetTokenProxyMessage
}>

export type SignBackgroundMessage =
    | Readonly<{
          type: "password-reset-requestToken"
          response: RequestResetTokenProxyResponse
      }>
    | Readonly<{ type: "error"; err: string }>
