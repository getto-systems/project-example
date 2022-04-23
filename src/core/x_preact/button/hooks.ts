import { useEffect, useState } from "preact/hooks"

export type SuccessState = "success-confirming" | "normal"

export function useSuccessState(isSuccess: boolean): SuccessState {
    const [state, setState] = useState<SuccessState>("normal")

    useEffect(() => {
        if (isSuccess) {
            setState("success-confirming")
            setTimeout(() => {
                setState("normal")
            }, 1000)
        }
    }, [isSuccess])

    return state
}
