import { newAuthzRepositoryPod } from "../kernel/infra/repository/authz"
import { newAuthnRepositoryPod } from "../kernel/infra/repository/authn"
import { newClearAuthTicketRemote } from "./infra/clear"

import { RemoteOutsideFeature } from "../../../../../ui/vendor/getto-application/infra/remote/feature"
import { RepositoryOutsideFeature } from "../../../../../ui/vendor/getto-application/infra/repository/feature"

import { ClearAuthTicketInfra } from "./infra"

type OutsideFeature = RemoteOutsideFeature & RepositoryOutsideFeature
export function newClearAuthTicketInfra(
    feature: OutsideFeature,
): ClearAuthTicketInfra {
    return {
        authn: newAuthnRepositoryPod(feature),
        authz: newAuthzRepositoryPod(feature),
        clear: newClearAuthTicketRemote(feature),
    }
}
