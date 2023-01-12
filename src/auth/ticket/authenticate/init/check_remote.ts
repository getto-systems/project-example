import { env } from "../../../../y_environment/ui/env"
import pb from "../../../../y_protobuf/proto.js"

import {
    fetchOptions,
    remoteCommonError,
    remoteInfraError,
} from "../../../../common/util/remote/init/helper"
import { decodeProtobuf } from "../../../../z_vendor/protobuf/helper"

import { Clock } from "../../../../common/util/clock/infra"
import { CheckAuthTicketRemote } from "../infra"

import { convertCheckRemote } from "../convert"

export function newCheckAuthTicketRemote(clock: Clock): CheckAuthTicketRemote {
    return async () => {
        try {
            const mock = false
            if (mock) {
                return {
                    success: true,
                    value: convertCheckRemote(clock, []),
                }
            }

            const opts = fetchOptions({
                serverURL: env.apiServerURL,
                path: "/auth/ticket",
                method: "PATCH",
            })
            const response = await fetch(opts.url, opts.options)

            if (!response.ok) {
                return remoteCommonError(response.status)
            }

            const message = decodeProtobuf(
                pb.auth.ticket.authenticate.service.AuthenticateWithTokenMaskedResponsePb,
                await response.text(),
            )

            return {
                success: true,
                value: convertCheckRemote(clock, message.granted?.permissions || []),
            }
        } catch (err) {
            return remoteInfraError(err)
        }
    }
}
