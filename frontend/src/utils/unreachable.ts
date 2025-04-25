export function unreachable(t: never): never {
  console.error("Unreachable:", t);
  throw t;
}
