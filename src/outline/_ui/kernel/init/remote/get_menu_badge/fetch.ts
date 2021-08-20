import { env } from "../../../../../../y_environment/_ui/env"
import { GetMenuBadgeResult_pb } from "../../../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../../z_details/_ui/remote/helper"
import { decodeProtobuf } from "../../../../../../../ui/vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../../z_details/_ui/remote/feature"

import { GetMenuBadgeRemote } from "../../../infra"

import { convertMenuBadgeRemote } from "../../../convert"

export function newGetMenuBadgeRemote(feature: RemoteOutsideFeature): GetMenuBadgeRemote {
    return async () => {
        try {
            // TODO api の実装が終わったらつなぐ
            const mock = true
            if (mock) {
                return { success: true, value: convertMenuBadgeRemote([]) }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/outline/menu/badge",
                method: "GET",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            const response = await fetch(opts.url, opts.options)

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            const result = decodeProtobuf(GetMenuBadgeResult_pb, await response.text())
            return {
                success: true,
                value: convertMenuBadgeRemote(
                    result.badge.map((item) => ({
                        path: item.path || "",
                        count: item.count || 0,
                    })),
                ),
            }
        } catch (err) {
            return remoteInfraError(err)
        }
    }
}
