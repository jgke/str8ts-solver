export interface ErrorProps {
  error: string;
}
export function Error(props: ErrorProps) {
  const { error } = props;
  return error && <div className="bg-error my-4 max-w-[600px] p-4">{error}</div>;
}
