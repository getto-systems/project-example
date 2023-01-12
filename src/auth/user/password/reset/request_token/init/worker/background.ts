import { WorkerBackgroundHandler } from "../../../../../../../z_vendor/getto-application/action/worker/background"

import { newRequestResetTokenInfra } from "../infra"

import { RequestResetTokenProxyMessage, RequestResetTokenProxyResponse } from "./message"

export function newRequestResetTokenWorkerHandler(
    postMessage: Post<RequestResetTokenProxyResponse>,
): WorkerBackgroundHandler<RequestResetTokenProxyMessage, RequestResetTokenProxyResponse> {
    const infra = newRequestResetTokenInfra()

    return async (message) => {
        switch (message.name) {
            case "request-token-remote":
                return post({
                    name: "request-token-remote",
                    id: message.id,
                    data: await infra.requestTokenRemote(message.params.fields),
                })
        }
    }

    function post(response: RequestResetTokenProxyResponse): RequestResetTokenProxyResponse {
        postMessage(response)
        return response
    }
}

interface Post<M> {
    (message: M): void
}
