import { env } from "../../../../../../y_environment/_ui/env"
import pb from "../../../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../../z_details/_ui/remote/helper"
import { encodeProtobuf } from "../../../../../../../ui/vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../../z_details/_ui/remote/feature"

import { NotifyUnexpectedErrorRemote } from "../../infra"

export function newNotifyUnexpectedErrorRemote(
    feature: RemoteOutsideFeature,
): NotifyUnexpectedErrorRemote {
    return async (err) => {
        try {
            const mock = false
            if (mock) {
                return { success: true, value: true }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/avail/unexpected-error",
                method: "POST",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            const response = await fetch(opts.url, {
                ...opts.options,
                body: encodeProtobuf(pb.avail.api.NotifyUnexpectedError_pb, (message) => {
                    message.json = JSON.stringify({ message: `${err}`, err })
                }),
            })

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            return { success: true, value: true }
        } catch (err) {
            return remoteInfraError(err)
        }
    }
}
