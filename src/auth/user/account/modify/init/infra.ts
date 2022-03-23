import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { newModifyAuthUserAccountRemote } from "./modify_remote"

import {  ModifyAuthUserAccountInfra } from "../action"

type OutsideFeature = RemoteOutsideFeature
export function newModifyAuthUserAccountInfra(feature: OutsideFeature): ModifyAuthUserAccountInfra {
    return {
        modifyUserRemote: newModifyAuthUserAccountRemote(feature),
    }
}
