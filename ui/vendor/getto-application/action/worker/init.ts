import { InitWorkerOutsideFeature } from "./infra"

export function newWorker(feature: InitWorkerOutsideFeature): Worker {
    const { webDocument } = feature
    const src = webDocument.currentScript?.getAttribute("src")
    if (!src) {
        throw new Error("invalid script src")
    }
    return new Worker(src.replace(/\.js$/, ".worker.js"))
}
