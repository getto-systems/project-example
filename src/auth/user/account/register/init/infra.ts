import { RemoteOutsideFeature } from "../../../../../z_lib/ui/remote/feature"

import { newRegisterAuthUserAccountRemote } from "./register_remote"

import {  RegisterAuthUserAccountInfra } from "../action"

type OutsideFeature = RemoteOutsideFeature
export function newRegisterAuthUserAccountInfra(feature: OutsideFeature): RegisterAuthUserAccountInfra {
    return {
        registerUserRemote: newRegisterAuthUserAccountRemote(feature),
    }
}
