import { newWorker } from "../../../ui/vendor/getto-application/action/worker/init"

import {
    newCommonOutsideFeature,
    CommonOutsideFeature,
    newForegroundOutsideFeature,
    ForegroundOutsideFeature,
} from "./common"

import { WorkerOutsideFeature } from "../../../ui/vendor/getto-application/action/worker/feature"

type WorkerForegroundOutsideFeature = ForegroundOutsideFeature & WorkerOutsideFeature
type WorkerBackgroundOutsideFeature = CommonOutsideFeature & WorkerOutsideFeature

export function newWorkerForegroundOutsideFeature(): WorkerForegroundOutsideFeature {
    return {
        ...newForegroundOutsideFeature(),
        worker: newWorker({
            webDocument: document,
        }),
    }
}
export function newWorkerBackgroundOutsideFeature(): WorkerBackgroundOutsideFeature {
    return {
        ...newCommonOutsideFeature(),
        worker: (self as unknown) as Worker,
    }
}
