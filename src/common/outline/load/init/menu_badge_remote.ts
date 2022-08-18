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

import { LoadMenuBadgeRemote, LoadMenuBadgeRemoteResult, MenuBadgeItem } from "../infra"

import { convertMenuBadgeRemote } from "../convert"
import { decodeOutlineMenuBadgePath } from "../../../../x_content/menu/badge"

export function newLoadMenuBadgeRemote(feature: RemoteOutsideFeature): LoadMenuBadgeRemote {
    return () => fetchRemote(feature)
}
async function fetchRemote(feature: RemoteOutsideFeature): Promise<LoadMenuBadgeRemoteResult> {
    try {
        const mock = false
        if (mock) {
            return { success: true, value: convertMenuBadgeRemote([]) }
        }

        const opts = fetchOptions({
            serverURL: env.apiServerURL,
            path: "/common/outline/menu-badge",
            method: "GET",
            headers: [[env.apiServerNonceHeader, generateNonce(feature)]],
        })
        const response = await fetch(opts.url, opts.options)

        if (!response.ok) {
            return remoteCommonError(response.status)
        }

        const message = decodeProtobuf(
            pb.common.outline.load.service.LoadMenuBadgeResponsePb,
            await response.text(),
        )
        return {
            success: true,
            value: convertMenuBadgeRemote(decodeItems(message.items)),
        }
    } catch (err) {
        return remoteInfraError(err)
    }
}

function decodeItems(
    items: readonly pb.common.outline.load.service.ILoadMenuBadgeEntryPb[],
): readonly MenuBadgeItem[] {
    const decoded: MenuBadgeItem[] = []

    items.forEach((item) => {
        if (item !== null && item !== undefined) {
            const path = decodeOutlineMenuBadgePath(item.path || "")
            if (path.success) {
                decoded.push({ path: path.path, count: item.count || 0 })
            }
        }
    })

    return decoded
}
