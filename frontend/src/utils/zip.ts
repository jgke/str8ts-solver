export function zip<T>(a: T[], b: T[]): [T, T][] {
  return a.map((x, i) => [x, b[i]]);
}
