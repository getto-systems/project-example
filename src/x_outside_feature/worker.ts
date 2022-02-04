import { newWorker } from "../z_vendor/getto-application/action/worker/init"

import {
    newCommonOutsideFeature,
    CommonOutsideFeature,
    newForegroundOutsideFeature,
    ForegroundOutsideFeature,
} from "./common"

import { WorkerOutsideFeature } from "../z_vendor/getto-application/action/worker/feature"

type WorkerForegroundOutsideFeature = ForegroundOutsideFeature & WorkerOutsideFeature
type WorkerBackgroundOutsideFeature = CommonOutsideFeature & WorkerOutsideFeature

export async function newWorkerForegroundOutsideFeature(): Promise<WorkerForegroundOutsideFeature> {
    return {
        ...newForegroundOutsideFeature(),
        worker: await newWorker({
            webDocument: document,
        }),
    }
}
export function newWorkerBackgroundOutsideFeature(): WorkerBackgroundOutsideFeature {
    return {
        ...newCommonOutsideFeature(),
        worker: self as unknown as Worker,
    }
}
