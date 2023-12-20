import { VNode } from "preact"

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export type PreactNode = VNode<any>
export type PreactContent = PreactEntry | readonly PreactEntry[]
type PreactEntry = string | number | PreactNode

export type PreactKey = string | number
