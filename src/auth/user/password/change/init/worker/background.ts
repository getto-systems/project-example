import { WorkerBackgroundHandler } from "../../../../../../../ui/vendor/getto-application/action/worker/background"

import { RemoteOutsideFeature } from "../../../../../../z_lib/ui/remote/feature"

import { newChangePasswordInfra } from "../infra"

import { ChangePasswordProxyMessage, ChangePasswordProxyResponse } from "./message"

type OutsideFeature = RemoteOutsideFeature
export function newChangePasswordWorkerHandler(
    feature: OutsideFeature,
    postMessage: Post<ChangePasswordProxyResponse>,
): WorkerBackgroundHandler<ChangePasswordProxyMessage, ChangePasswordProxyResponse> {
    const infra = newChangePasswordInfra(feature)

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
