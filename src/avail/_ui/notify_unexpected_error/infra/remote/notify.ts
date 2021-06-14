import { env } from "../../../../../y_environment/_ui/env"
import { NotifyUnexpectedError_pb } from "../../../y_protobuf/api_pb.js"

import {
    convertRemote,
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../z_details/_ui/remote/helper"
import { encodeProtobuf } from "../../../../../../ui/vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../z_details/_ui/remote/feature"

import { NotifyUnexpectedErrorRemotePod } from "../../infra"

import { ApiCommonError, ApiResult } from "../../../../../z_details/_ui/api/data"

export function newNotifyUnexpectedErrorRemote(
    feature: RemoteOutsideFeature,
): NotifyUnexpectedErrorRemotePod {
    type NotifyResult = ApiResult<true, ApiCommonError>

    return convertRemote(async (err: unknown): Promise<NotifyResult> => {
        try {
            const mock = true
            if (mock) {
                // TODO api の実装が終わったらつなぐ
                return { success: true, value: true }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/avail/error/unexpected",
                method: "POST",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            const response = await fetch(opts.url, {
                ...opts.options,
                body: encodeProtobuf(NotifyUnexpectedError_pb, (message) => {
                    message.json = JSON.stringify(err)
                }),
            })

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            return { success: true, value: true }
        } catch (err) {
            return remoteInfraError(err)
        }
    })
}
