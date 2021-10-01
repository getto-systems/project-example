import {
    RequestResetTokenProfileProxyMessage,
    RequestResetTokenProfileProxyResponse,
} from "../../../../../../src/auth/user/password/reset/action_request_token_profile/init/worker/message"

export type ProfileForegroundMessage = Readonly<{
    type: "password-reset-requestToken"
    message: RequestResetTokenProfileProxyMessage
}>

export type ProfileBackgroundMessage =
    | Readonly<{
          type: "password-reset-requestToken"
          response: RequestResetTokenProfileProxyResponse
      }>
    | Readonly<{ type: "error"; err: string }>
