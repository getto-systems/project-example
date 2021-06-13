import { env } from "../../../../../y_environment/_ui/env"
import { NotifyUnexpectedError_pb } from "../../../y_protobuf/api_pb.js"

import {
    remoteFeature,
    convertRemote,
} from "../../../../../../ui/vendor/getto-application/infra/remote/helper"
import { encodeProtobuf } from "../../../../../../ui/vendor/protobuf/helper"
import { apiInfraError, apiRequest, apiStatusError } from "../../../../../z_details/_ui/api/helper"

import { RemoteOutsideFeature } from "../../../../../../ui/vendor/getto-application/infra/remote/feature"

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

            const request = apiRequest(
                remoteFeature(env.apiServerURL, feature),
                "/avail/error/unexpected",
                "POST",
            )
            const response = await fetch(request.url, {
                ...request.options,
                body: encodeProtobuf(NotifyUnexpectedError_pb, (message) => {
                    message.json = JSON.stringify(err)
                }),
            })

            if (!response.ok) {
                return apiStatusError(response.status)
            }

            return { success: true, value: true }
        } catch (err) {
            return apiInfraError(err)
        }
    })
}
