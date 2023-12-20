import { WorkerBackgroundHandler } from "../../../../../../common/util/worker/background"

import { newChangePasswordInfra } from "../infra"

import { ChangePasswordProxyMessage, ChangePasswordProxyResponse } from "./message"

export function newChangePasswordWorkerHandler(
    postMessage: Post<ChangePasswordProxyResponse>,
): WorkerBackgroundHandler<ChangePasswordProxyMessage, ChangePasswordProxyResponse> {
    const infra = newChangePasswordInfra()

    return async (message) => {
        switch (message.name) {
            case "change-password-remote":
                return post({
                    name: "change-password-remote",
                    id: message.id,
                    data: await infra.changePasswordRemote(message.params.fields),
                })
        }
    }

    function post(response: ChangePasswordProxyResponse): ChangePasswordProxyResponse {
        postMessage(response)
        return response
    }
}

interface Post<M> {
    (message: M): void
}
