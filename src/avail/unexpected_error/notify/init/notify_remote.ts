import { env } from "../../../../y_environment/ui/env"
import pb from "../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../common/util/remote/init/helper"
import { encodeProtobuf } from "../../../../z_vendor/protobuf/helper"

import { NotifyUnexpectedErrorRemote } from "../infra"

export function newNotifyUnexpectedErrorRemote(): NotifyUnexpectedErrorRemote {
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
            })
            const response = await fetch(opts.url, {
                ...opts.options,
                body: encodeProtobuf(
                    pb.avail.unexpected_error.notify.service.NotifyRequestPb,
                    (message) => {
                        message.err = JSON.stringify({ type: "UI ERROR", message: `${err}`, err })
                    },
                ),
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
