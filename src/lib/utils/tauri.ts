import type { Result } from "./bindings";

type TauriCommand<A extends unknown[], D, E> = (
  ...args: A
) => Promise<Result<D, E>>;

type CommandResponse<TData, TError> =
  | {
      data: TData;
      error?: null;
    }
  | {
      data?: null;
      error: TError;
    };

export function wrapTauriCommand<TArgs extends unknown[], TData, TError>(
  command: TauriCommand<TArgs, TData, TError>
): (...args: TArgs) => Promise<CommandResponse<TData, TError>> {
  return async (...args: TArgs) => {
    const result = await command(...args);
    if (result.status === "ok") {
      return { data: result.data, error: null };
    } else {
      return { data: null, error: result.error };
    }
  };
}
