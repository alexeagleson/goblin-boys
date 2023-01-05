import { LOG_LEVEL } from "./config";

export const assertNever = (value: never, noThrow?: boolean): never => {
  if (noThrow) {
    return value;
  }

  throw new Error(
    `Unhandled discriminated union member: ${JSON.stringify(value)}`
  );
};

export const log = {
  trace: (...params: Parameters<typeof console.log>) => {
    if (LOG_LEVEL === "trace") console.log("TRACE", ...params);
  },
};
