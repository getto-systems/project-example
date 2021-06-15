import { newAuthzRepositoryPod } from "../kernel/infra/repository/authz"
import { newAuthnRepository } from "../kernel/infra/repository/authn"
import { newClearAuthTicketRemote } from "./infra/clear"

import { RemoteOutsideFeature } from "../../../../z_details/_ui/remote/feature"
import { RepositoryOutsideFeature } from "../../../../z_details/_ui/repository/feature"

import { ClearAuthTicketInfra } from "./infra"

type OutsideFeature = RemoteOutsideFeature & RepositoryOutsideFeature
export function newClearAuthTicketInfra(
    feature: OutsideFeature,
): ClearAuthTicketInfra {
    return {
        authn: newAuthnRepository(feature),
        authz: newAuthzRepositoryPod(feature),
        clear: newClearAuthTicketRemote(feature),
    }
}
