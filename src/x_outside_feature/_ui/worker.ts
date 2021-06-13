import { newWorker } from "../../../ui/vendor/getto-application/action/worker/init"

import {
    commonOutsideFeature,
    CommonOutsideFeature,
    foregroundOutsideFeature,
    ForegroundOutsideFeature,
} from "./common"

import { WorkerOutsideFeature } from "../../../ui/vendor/getto-application/action/worker/feature"

type WorkerForegroundOutsideFeature = ForegroundOutsideFeature & WorkerOutsideFeature
type WorkerBackgroundOutsideFeature = CommonOutsideFeature & WorkerOutsideFeature

export function workerForegroundOutsideFeature(): WorkerForegroundOutsideFeature {
    return {
        ...foregroundOutsideFeature(),
        worker: newWorker({
            webDocument: document,
        }),
    }
}
export function workerBackgroundOutsideFeature(): WorkerBackgroundOutsideFeature {
    return {
        ...commonOutsideFeature(),
        worker: (self as unknown) as Worker,
    }
}
