import { env } from "../../../../../../../y_environment/_ui/env"
import pb from "../../../../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../../../../z_details/_ui/remote/helper"
import { decodeProtobuf } from "../../../../../../../../ui/vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../../../../z_details/_ui/remote/feature"

import { GetMenuBadgeRemote } from "../../../infra"

import { convertMenuBadgeRemote } from "../../../convert"

export function newGetMenuBadgeRemote(feature: RemoteOutsideFeature): GetMenuBadgeRemote {
    return async () => {
        try {
            const mock = false
            if (mock) {
                return { success: true, value: convertMenuBadgeRemote([]) }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/example/outline/menu-badge",
                method: "GET",
                headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
            })
            const response = await fetch(opts.url, opts.options)

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            const result = decodeProtobuf(
                pb.example.outline.api.GetMenuBadgeResult_pb,
                await response.text(),
            )
            return {
                success: true,
                value: convertMenuBadgeRemote([
                    {
                        path: "index.html",
                        count: result.index,
                    },
                ]),
            }
        } catch (err) {
            return remoteInfraError(err)
        }
    }
}
