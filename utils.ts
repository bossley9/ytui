/**
 * Returns command line args supplied in the form of an array
 */
export const parseArgs = (): string[] => {
  // @ts-ignore
  // Deno is undefined by tsserver
  return Deno.args;
};

/**
 * Simple wrapper in case encoding needs to be adjusted
 */
export const urlEncode = (url: string): string => {
  return encodeURIComponent(url);
};
