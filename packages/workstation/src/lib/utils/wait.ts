/**
 * Inserts a wait to a function call, delaying its execution until after `waitFor` milliseconds
 * have elapsed since the last time it was invoked.
 *
 * @param func - The function to insert a wait within execution.
 * @param waitFor - The number of milliseconds to delay before execution starts.
 *                  Also used as the threshold - if execution takes >= waitFor ms, onLoading is called.
 * @param onLoading - Optional callback that fires if func is in progress after waitFor milliseconds.
 * @returns A waited version of the function that returns a Promise of the result.
 */
export function wait<T extends (...args: any[]) => Promise<any>>(
  func: T,
  waitFor: number,
  onLoading?: () => void,
): (...args: Parameters<T>) => Promise<ReturnType<T>> {
  let timeout: ReturnType<typeof setTimeout>;
  if (onLoading) {
    timeout = setTimeout(() => onLoading(), waitFor);
  }

  return async (...args: Parameters<T>): Promise<ReturnType<T>> => {
    const result = await func(...args);

    if (timeout) {
      clearTimeout(timeout);
    }

    return result;
  };
}
