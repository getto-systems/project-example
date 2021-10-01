import { InitWorkerOutsideFeature } from "./feature"

export async function newWorker(feature: InitWorkerOutsideFeature): Promise<Worker> {
    const { webDocument } = feature
    const src = webDocument.currentScript?.getAttribute("src")
    if (!src) {
        throw new Error("invalid script src")
    }

    const response = await fetch(src.replace(/\.js$/, ".worker.js"))
    const code = new Blob([await response.text()], { type: "application/javascript" })
    return new Worker(URL.createObjectURL(code))
}
