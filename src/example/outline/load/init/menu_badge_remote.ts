import { env } from "../../../../y_environment/ui/env"
import pb from "../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    generateNonce,
    remoteCommonError,
    remoteInfraError,
} from "../../../../z_lib/ui/remote/init/helper"
import { decodeProtobuf } from "../../../../z_vendor/protobuf/helper"

import { RemoteOutsideFeature } from "../../../../z_lib/ui/remote/feature"

import { LoadMenuBadgeRemote } from "../infra"

import { convertMenuBadgeRemote } from "../convert"

export function newLoadMenuBadgeRemote(feature: RemoteOutsideFeature): LoadMenuBadgeRemote {
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

            const message = decodeProtobuf(
                pb.example.outline.service.LoadMenuBadgeResponsePb,
                await response.text(),
            )
            return {
                success: true,
                value: convertMenuBadgeRemote([{ path: "index.html", count: message.index }]),
            }
        } catch (err) {
            return remoteInfraError(err)
        }
    }
}
