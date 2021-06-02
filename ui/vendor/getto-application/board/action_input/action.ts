import { InputBoardValueType } from "../input/data";
import { InputBoardValueAction } from "./core/action";

export type InputBoardValueResource = Readonly<{
    type: InputBoardValueType
    input: InputBoardValueAction
}>
